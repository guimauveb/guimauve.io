use {
    super::{articles::Article, chapters::Chapter},
    crate::{
        diesel::{BelongingToDsl, PgConnection, QueryDsl, RunQueryDsl},
        schema::contents,
        types::{content_type::ContentType, language::Language},
        API_URL,
    },
    serde::{Deserialize, Serialize},
};

#[cfg(feature = "editable")]
use {
    super::articles::ArticleRepresentation,
    crate::{
        code::highlight_code,
        interfaces::{Status, TAPIResponse},
        schema::chapters,
    },
    diesel::{connection::Connection, pg::expression::dsl::any, ExpressionMethods},
};

#[derive(
    Debug, Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, AsChangeset,
)]
#[belongs_to(parent = "Article")]
#[belongs_to(parent = "Chapter")]
#[table_name = "contents"]
pub struct Content {
    pub id: i32,
    pub article_id: i32,
    pub chapter_id: i32,
    pub index: i32,
    pub content_type: ContentType,
    pub content: String,
    pub language: Option<Language>,
    pub highlighted_code: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentRepresentation {
    pub id: i32,
    pub chapter_id: i32,
    pub article_id: i32,
    pub index: i32,
    pub content_type: ContentType,
    pub content: String,
    pub language: Option<Language>,
    pub highlighted_code: Option<String>,
    pub url: Option<String>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "contents"]
pub struct NewContent<'a> {
    pub article_id: i32,
    pub chapter_id: i32,
    pub index: i32,
    pub content_type: ContentType,
    pub content: &'a str,
    pub language: Option<Language>,
    pub highlighted_code: Option<String>,
    pub url: Option<&'a str>,
}

impl Content {
    fn into_representation(self) -> ContentRepresentation {
        ContentRepresentation {
            id: self.id,
            article_id: self.article_id,
            chapter_id: self.chapter_id,
            index: self.index,
            content: match self.content_type {
                ContentType::Image => API_URL.to_owned() + &self.content,
                _ => self.content,
            },
            content_type: self.content_type,
            language: self.language,
            highlighted_code: self.highlighted_code,
            url: self.url,
        }
    }

    #[cfg(feature = "editable")]
    pub fn update(
        id: &i32,
        updated_content: Content,
        connection: &PgConnection,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let article_id = updated_content.article_id;
        let mut content = updated_content;
        if content.content_type == ContentType::Code {
            let language = content
                .language
                .as_ref()
                .expect("Code content should specify a language!")
                .to_string();
            content.highlighted_code = Some(highlight_code(&content.content, &language));
        }
        // TODO - Check if we can retrn Article
        diesel::update(contents::table.find(id))
            .set(content)
            .execute(connection)?;

        Article::get(&article_id, connection)
    }

    #[cfg(feature = "editable")]
    pub fn add(
        new_content: NewContent,
        connection: &PgConnection,
    ) -> Result<i32, diesel::result::Error> {
        let new_content_id = connection.transaction::<i32, diesel::result::Error, _>(|| {
            let chapter = chapters::table
                .find(new_content.chapter_id)
                .first::<Chapter>(connection)
                .expect("Error loading chapter.");

            let contents_ids = Content::belonging_to(&chapter)
                .select(contents::id)
                .load::<i32>(connection)
                .expect("Error loading contents.");

            diesel::update(contents::table.filter(contents::id.eq(any(contents_ids))))
                .filter(contents::index.ge(&new_content.index))
                .set(contents::index.eq(contents::index + 1))
                .execute(connection)
                .expect("An error occured while incrementing contents ids.");

            let new_content_id = diesel::insert_into(contents::table)
                .values(&new_content)
                .returning(contents::id)
                .get_result(connection)
                .expect("Could not insert content.");

            Ok(new_content_id)
        })?;

        Ok(new_content_id)
    }

    #[cfg(feature = "editable")]
    pub fn delete(
        id: &i32,
        connection: &PgConnection,
    ) -> Result<TAPIResponse<()>, diesel::result::Error> {
        connection.transaction::<(), diesel::result::Error, _>(|| {
            let content = contents::table
                .find(id)
                .first::<Content>(connection)
                .expect("Error loading chapter.");

            let chapter = chapters::table
                .find(content.chapter_id)
                .load::<Chapter>(connection)
                .expect("Error loading chapter.");

            let contents_ids: Vec<i32> = Content::belonging_to(&chapter)
                .select(contents::id)
                .load::<i32>(connection)
                .expect("Error loading contents ids.");

            diesel::update(contents::table.filter(contents::id.eq(any(contents_ids))))
                .filter(contents::index.gt(content.index))
                .set(contents::index.eq(contents::index - 1))
                .execute(connection)
                .expect("An error occured while decrementing contents ids.");

            diesel::delete(contents::table.filter(contents::id.eq(id)))
                .execute(connection)
                .expect("Could not delete content.");

            Ok(())
        })?;

        Ok(TAPIResponse {
            status: Status::Success,
            content: None,
        })
    }

    pub fn belonging_to_chapter(
        chapter: &Chapter,
        connection: &PgConnection,
    ) -> Result<Vec<ContentRepresentation>, diesel::result::Error> {
        let contents = Content::belonging_to(chapter)
            .order_by(contents::index)
            .load::<Content>(connection)
            .expect("Error loading contents.");

        Ok(contents
            .into_iter()
            .map(|content| content.into_representation())
            .collect())
    }
}
