#[cfg(feature = "editable")]
use {
    super::{
        chapters::NewChapter,
        contents::{Content, NewContent},
    },
    crate::interfaces::{IArticleHeader, Status, TAPIResponse},
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
    serde_derive::{Deserialize, Serialize},
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
}

type ArticleColumns = (
    articles::id,
    articles::title,
    articles::pub_date,
    articles::published,
    articles::headline,
    articles::image,
    articles::image_credits,
);

pub const ARTICLE_COLUMNS: ArticleColumns = (
    articles::id,
    articles::title,
    articles::pub_date,
    articles::published,
    articles::headline,
    articles::image,
    articles::image_credits,
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
            .load::<Tag>(connection)
            .expect("Error loading tags.");

        Ok(tags)
    }

    pub fn chapters(
        &self,
        connection: &PgConnection,
    ) -> Result<Vec<ChapterRepresentation>, diesel::result::Error> {
        Chapter::belonging_to_article(connection, self)
    }

    // TODO - Use Into<ArticleRepresentation> instead?
    fn to_representation(self, connection: &PgConnection) -> ArticleRepresentation {
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
        }
    }

    pub fn find(
        connection: &PgConnection,
        id: &i32,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let article = articles::table
            .select(ARTICLE_COLUMNS)
            .find(id)
            .first::<Article>(connection)?;

        Ok(article.to_representation(connection))
    }

    #[cfg(feature = "editable")]
    pub fn add(
        connection: &PgConnection,
        new_article: NewArticle,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let inserted_article_id: i32 = diesel::insert_into(articles::table)
            .values(&new_article.article_header)
            .returning(articles::id)
            .get_result(connection)
            .expect("Could not insert article.");

        for chapter_form in new_article.chapters {
            let inserted_chapter_id = Chapter::add(
                connection,
                NewChapter {
                    article_id: inserted_article_id,
                    ..chapter_form.chapter
                },
            )?;
            for content in chapter_form.contents {
                Content::add(
                    connection,
                    NewContent {
                        article_id: inserted_article_id,
                        chapter_id: inserted_chapter_id,
                        ..content
                    },
                )?;
            }
        }

        Article::find(connection, &inserted_article_id)
    }

    // TODO - Check return type
    #[cfg(feature = "editable")]
    pub fn delete(
        connection: &PgConnection,
        id: &i32,
    ) -> Result<TAPIResponse<()>, diesel::result::Error> {
        diesel::delete(articles::table.filter(articles::id.eq(id))).execute(connection)?;

        Ok(TAPIResponse {
            status: Status::Success,
            content: Some(()),
        })
    }

    #[cfg(feature = "editable")]
    pub fn update(
        connection: &PgConnection,
        id: &i32,
        updated_header: IArticleHeader, // TODO - Check type
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        diesel::update(articles::table.find(id))
            .set(&Article {
                id: updated_header.article_id,
                title: updated_header.title,
                pub_date: updated_header.pub_date,
                published: updated_header.published,
                headline: updated_header.headline,
                image: updated_header.image,
                image_credits: updated_header.image_credits,
            })
            .execute(connection)
            .expect("Could not update article.");

        Article::find(connection, id)
    }

    #[cfg(feature = "editable")]
    pub fn publish(
        connection: &PgConnection,
        id: &i32,
        published: &bool,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        diesel::update(articles::table.filter(articles::id.eq(id)))
            .set(articles::published.eq(published))
            .execute(connection)
            .expect("An error occured while updating the article.");

        Article::find(connection, id)
    }

    pub fn list(
        connection: &PgConnection,
    ) -> Result<HashMap<i32, ArticleRepresentation>, diesel::result::Error> {
        let articles = articles::table
            .select(ARTICLE_COLUMNS)
            .load(connection)
            .expect("Could not load articles.");

        let results: HashMap<i32, ArticleRepresentation> = articles
            .into_iter()
            .map(|article: Article| (article.id, article.to_representation(connection)))
            .collect();

        Ok(results)
    }

    pub fn search(
        connection: &PgConnection,
        query: &str,
    ) -> Result<HashMap<i32, ArticleRepresentation>, diesel::result::Error> {
        let articles = match INCLUDE_UNPUBLISHED_ARTICLES {
            "true" => articles::table
                .select(ARTICLE_COLUMNS)
                .filter(articles::text_searchable_article.matches(plainto_tsquery(&query)))
                .load::<Article>(connection)
                .expect("Error loading articles."),
            _ => articles::table
                .select(ARTICLE_COLUMNS)
                .filter(articles::published.eq(true))
                .filter(articles::text_searchable_article.matches(plainto_tsquery(&query)))
                .load::<Article>(connection)
                .expect("Error loading articles."),
        };

        let results: HashMap<i32, ArticleRepresentation> = articles
            .into_iter()
            .map(|article: Article| (article.id, article.to_representation(connection)))
            .collect();

        Ok(results)
    }

    pub fn tagged(
        connection: &PgConnection,
        label: &str, // TODO - Tag
    ) -> Result<HashMap<i32, ArticleRepresentation>, diesel::result::Error> {
        let tag = tags::table
            .filter(tags::label.eq(label))
            .first::<Tag>(connection)?;

        let article_ids = ArticleTag::belonging_to(&tag)
            .select(article_tags::article_id)
            .load::<i32>(connection)
            .expect("Error loading article tags.");
        let articles = articles::table
            .select(ARTICLE_COLUMNS)
            .filter(articles::id.eq(any(article_ids)))
            .load::<Article>(connection)
            .expect("Error loading articles.");

        // TODO - Use a vec
        let results: HashMap<i32, ArticleRepresentation> = articles
            .into_iter()
            .map(|article: Article| (article.id, article.to_representation(connection)))
            .collect();

        Ok(results)
    }
}
