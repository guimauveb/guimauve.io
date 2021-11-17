#[cfg(feature = "editable")]
use {
    super::{
        chapters::NewChapter,
        contents::{Content, NewContent},
    },
    crate::interfaces::{Status, TAPIResponse},
    diesel::connection::Connection,
};

use {
    super::{
        chapters::{Chapter, ChapterRepresentation, NewChapterForm},
        tags::Tag,
    },
    crate::{
        diesel::{
            pg::expression::dsl::any, BelongingToDsl, ExpressionMethods, PgConnection, QueryDsl,
            RunQueryDsl,
        },
        diesel_full_text_search::{plainto_tsquery, TsVectorExtensions},
        schema::{article_tags, articles, tags},
        API_URL, INCLUDE_UNPUBLISHED_ARTICLES,
    },
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

#[derive(
    Debug, Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, AsChangeset,
)]
#[belongs_to(Article)]
#[belongs_to(Tag)]
pub struct ArticleTag {
    pub id: i32,
    pub article_id: i32,
    pub tag_id: i32,
}

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Clone, AsChangeset)]
#[table_name = "articles"]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub image_credits: Option<String>,
    pub updated: Option<chrono::NaiveDateTime>,
}

type ArticleColumns = (
    articles::id,
    articles::title,
    articles::pub_date,
    articles::published,
    articles::headline,
    articles::image,
    articles::image_credits,
    articles::updated,
);

pub const ARTICLE_COLUMNS: ArticleColumns = (
    articles::id,
    articles::title,
    articles::pub_date,
    articles::published,
    articles::headline,
    articles::image,
    articles::image_credits,
    articles::updated,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleRepresentation {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub image_credits: Option<String>,
    pub updated: Option<chrono::NaiveDateTime>,
    pub tags: Vec<Tag>,
    pub chapters: Vec<ChapterRepresentation>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "articles"]
pub struct NewArticleHeader<'a> {
    pub title: &'a str,
    pub headline: &'a str,
    pub published: bool,
    pub image: &'a str,
    pub image_credits: Option<&'a str>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "article_tags"]
pub struct NewArticleTag {
    pub article_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewArticle<'a> {
    #[serde(borrow)]
    pub article_header: NewArticleHeader<'a>,
    pub tags: Vec<NewArticleTag>,
    #[serde(borrow)]
    pub chapters: Vec<NewChapterForm<'a>>,
}

impl Article {
    fn tags(&self, connection: &PgConnection) -> Result<Vec<Tag>, diesel::result::Error> {
        let tags_ids = ArticleTag::belonging_to(self).select(article_tags::tag_id);
        let tags = tags::table
            .filter(tags::id.eq(any(tags_ids)))
            .load::<Tag>(connection)?;

        Ok(tags)
    }

    fn chapters(
        &self,
        connection: &PgConnection,
    ) -> Result<Vec<ChapterRepresentation>, diesel::result::Error> {
        Chapter::belonging_to_article(self, connection)
    }

    fn into_representation(self, connection: &PgConnection) -> ArticleRepresentation {
        ArticleRepresentation {
            tags: self.tags(connection).expect("Error loading article tags."),
            chapters: self.chapters(connection).expect("Error loading chapters."),
            id: self.id,
            title: self.title,
            pub_date: self.pub_date,
            published: self.published,
            headline: self.headline,
            image: API_URL.to_owned() + &self.image,
            image_credits: self.image_credits,
            updated: self.updated,
        }
    }

    pub fn get(
        id: i32,
        connection: &PgConnection,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let article = articles::table
            .select(ARTICLE_COLUMNS)
            .find(id)
            .first::<Article>(connection)?;

        Ok(article.into_representation(connection))
    }

    #[cfg(feature = "editable")]
    pub fn add(
        new_article: &NewArticle,
        connection: &PgConnection,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let inserted_article =
            connection.transaction::<Article, diesel::result::Error, _>(|| {
                let inserted_article = diesel::insert_into(articles::table)
                    .values(&new_article.article_header)
                    .returning(ARTICLE_COLUMNS)
                    .get_result::<Article>(connection)?;

                for new_chapter in &new_article.chapters {
                    let inserted_chapter_id = Chapter::add(
                        &NewChapter {
                            article_id: inserted_article.id,
                            ..new_chapter.chapter
                        },
                        connection,
                    )?;

                    // TODO - Pass highlighted_code/content_type/language by ref
                    for new_content in &new_chapter.contents {
                        Content::add(
                            &NewContent {
                                article_id: inserted_article.id,
                                chapter_id: inserted_chapter_id,
                                //..*new_content
                                ..(*new_content).clone()
                            },
                            connection,
                        )?;
                    }
                }

                Ok(inserted_article)
            })?;

        Ok(inserted_article.into_representation(connection))
    }

    #[cfg(feature = "editable")]
    pub fn delete(
        id: i32,
        connection: &PgConnection,
    ) -> Result<TAPIResponse<()>, diesel::result::Error> {
        diesel::delete(articles::table.filter(articles::id.eq(id))).execute(connection)?;

        Ok(TAPIResponse {
            status: Status::Success,
            content: None,
        })
    }

    #[cfg(feature = "editable")]
    pub fn update(
        id: i32,
        updated_article: &Article,
        connection: &PgConnection,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let article = diesel::update(articles::table.find(id))
            .set(updated_article)
            .returning(ARTICLE_COLUMNS)
            .get_result::<Article>(connection)?;

        Ok(article.into_representation(connection))
    }

    #[cfg(feature = "editable")]
    pub fn publish(
        id: i32,
        published: bool,
        connection: &PgConnection,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let article = diesel::update(articles::table.filter(articles::id.eq(id)))
            .set(articles::published.eq(published))
            .returning(ARTICLE_COLUMNS)
            .get_result::<Article>(connection)?;

        Ok(article.into_representation(connection))
    }

    pub fn list(
        connection: &PgConnection,
    ) -> Result<HashMap<i32, ArticleRepresentation>, diesel::result::Error> {
        let articles = articles::table.select(ARTICLE_COLUMNS).load(connection)?;

        let results: HashMap<i32, ArticleRepresentation> = articles
            .into_iter()
            .map(|article: Article| (article.id, article.into_representation(connection)))
            .collect();

        Ok(results)
    }

    pub fn search(
        query: &str,
        connection: &PgConnection,
    ) -> Result<HashMap<i32, ArticleRepresentation>, diesel::result::Error> {
        let articles = match INCLUDE_UNPUBLISHED_ARTICLES {
            "true" => articles::table
                .select(ARTICLE_COLUMNS)
                .filter(articles::text_searchable_article.matches(plainto_tsquery(&query)))
                .load::<Article>(connection)?,

            _ => articles::table
                .select(ARTICLE_COLUMNS)
                .filter(articles::published.eq(true))
                .filter(articles::text_searchable_article.matches(plainto_tsquery(&query)))
                .load::<Article>(connection)?,
        };

        let results: HashMap<i32, ArticleRepresentation> = articles
            .into_iter()
            .map(|article: Article| (article.id, article.into_representation(connection)))
            .collect();

        Ok(results)
    }

    pub fn tagged(
        tag: &str, // TODO - Tag
        connection: &PgConnection,
    ) -> Result<HashMap<i32, ArticleRepresentation>, diesel::result::Error> {
        let tag = tags::table
            .filter(tags::label.eq(tag))
            .first::<Tag>(connection)?;

        let article_ids = ArticleTag::belonging_to(&tag)
            .select(article_tags::article_id)
            .load::<i32>(connection)?;
        let articles = articles::table
            .select(ARTICLE_COLUMNS)
            .filter(articles::id.eq(any(article_ids)))
            .load::<Article>(connection)?;

        // TODO - Use a vec
        let results: HashMap<i32, ArticleRepresentation> = articles
            .into_iter()
            .map(|article: Article| (article.id, article.into_representation(connection)))
            .collect();

        Ok(results)
    }
}
