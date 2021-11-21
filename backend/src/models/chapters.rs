use {
    super::{
        articles::Article,
        contents::{Content, ContentRepresentation, NewContent},
    },
    crate::{
        diesel::{BelongingToDsl, PgConnection, QueryDsl, RunQueryDsl},
        schema::chapters,
    },
    serde::{Deserialize, Serialize},
};

#[cfg(feature = "editable")]
use {
    super::articles::{ArticleRepresentation, ARTICLE_COLUMNS},
    crate::{
        interfaces::{Status, TAPIResponse},
        schema::articles,
    },
    diesel::{connection::Connection, pg::expression::dsl::any, ExpressionMethods},
};

#[derive(
    Debug, Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, AsChangeset,
)]
#[belongs_to(parent = "Article")]
#[table_name = "chapters"]
pub struct Chapter {
    pub id: i32,
    pub article_id: i32,
    pub index: i32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterRepresentation {
    pub id: i32,
    pub article_id: i32,
    pub index: i32,
    pub title: String,
    pub contents: Vec<ContentRepresentation>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "chapters"]
pub struct NewChapter<'a> {
    pub article_id: i32,
    pub index: i32,
    pub title: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewChapterForm<'a> {
    #[serde(borrow)]
    pub chapter: NewChapter<'a>,
    #[serde(borrow)]
    pub contents: Vec<NewContent<'a>>,
}

impl Chapter {
    fn contents(
        &self,
        connection: &PgConnection,
    ) -> Result<Vec<ContentRepresentation>, diesel::result::Error> {
        Content::belonging_to_chapter(self, connection)
    }

    fn into_representation(self, connection: &PgConnection) -> ChapterRepresentation {
        ChapterRepresentation {
            contents: self.contents(connection).unwrap_or_default(),
            id: self.id,
            article_id: self.article_id,
            index: self.index,
            title: self.title,
        }
    }

    #[cfg(feature = "editable")]
    pub fn delete(
        chapter_id: i32,
        connection: &PgConnection,
    ) -> Result<TAPIResponse<()>, diesel::result::Error> {
        connection.transaction::<(), diesel::result::Error, _>(|| {
            let chapter = chapters::table
                .find(chapter_id)
                .first::<Chapter>(connection)?;

            let article = articles::table
                .find(chapter.article_id)
                .select(ARTICLE_COLUMNS)
                .load::<Article>(connection)?;

            let chapters_ids: Vec<i32> = Chapter::belonging_to(&article)
                .select(chapters::id)
                .load::<i32>(connection)?;

            diesel::update(chapters::table.filter(chapters::id.eq(any(chapters_ids))))
                .filter(chapters::index.gt(chapter.index))
                .set(chapters::index.eq(chapters::index - 1))
                .execute(connection)?;

            diesel::delete(chapters::table.filter(chapters::id.eq(chapter_id)))
                .execute(connection)?;

            Ok(())
        })?;

        Ok(TAPIResponse {
            status: Status::Success,
            content: None,
        })
    }

    #[cfg(feature = "editable")]
    pub fn add(
        new_chapter: &NewChapter,
        connection: &PgConnection,
    ) -> Result<i32, diesel::result::Error> {
        let new_chapter_id = connection.transaction::<i32, diesel::result::Error, _>(|| {
            let article = articles::table
                .find(&new_chapter.article_id)
                .select(ARTICLE_COLUMNS)
                .first::<Article>(connection)?;

            let chapters_ids = Chapter::belonging_to(&article)
                .select(chapters::id)
                .load::<i32>(connection)?;

            diesel::update(chapters::table.filter(chapters::id.eq(any(chapters_ids))))
                .filter(chapters::index.ge(&new_chapter.index))
                .set(chapters::index.eq(chapters::index + 1))
                .execute(connection)?;

            let new_chapter_id = diesel::insert_into(chapters::table)
                .values(new_chapter)
                .returning(chapters::id)
                .get_result(connection)?;

            Ok(new_chapter_id)
        })?;

        Ok(new_chapter_id)
    }

    #[cfg(feature = "editable")]
    pub fn update(
        id: i32,
        updated_chapter: &Chapter,
        connection: &PgConnection,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let article_id = updated_chapter.article_id;
        diesel::update(chapters::table.find(id))
            .set(updated_chapter)
            .execute(connection)?;

        Article::get(article_id, connection)
    }

    pub fn belonging_to_article(
        article: &Article,
        connection: &PgConnection,
    ) -> Result<Vec<ChapterRepresentation>, diesel::result::Error> {
        let chapters = Self::belonging_to(article)
            .order_by(chapters::index)
            .load::<Self>(connection)?;

        Ok(chapters
            .into_iter()
            .map(|chapter| chapter.into_representation(connection))
            .collect())
    }
}
