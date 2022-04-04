use {
    super::{articles::Article, chapters::Chapter, from_model::FromModel},
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

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
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

impl FromModel<Content> for ContentRepresentation {
    fn from_model(content: Content, _: Option<&PgConnection>) -> Self {
        Self {
            id: content.id,
            article_id: content.article_id,
            chapter_id: content.chapter_id,
            index: content.index,
            content: match content.content_type {
                ContentType::Image => API_URL.to_owned() + &content.content,
                _ => content.content,
            },
            content_type: content.content_type,
            language: content.language,
            highlighted_code: content.highlighted_code,
            url: content.url,
        }
    }
}

impl Content {
    #[cfg(feature = "editable")]
    pub fn update(
        id: i32,
        mut content: Content,
        connection: &PgConnection,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let article_id = content.article_id;
        if content.content_type == ContentType::Code {
            let language = content
                .language
                .as_ref()
                .expect("Code content should specify a language!")
                .to_string();
            content.highlighted_code = Some(highlight_code(&content.content, &language));
        }
        diesel::update(contents::table.find(id))
            .set(content)
            .execute(connection)?;

        Article::get(article_id, connection)
    }

    #[cfg(feature = "editable")]
    pub fn add(
        new_content: &NewContent,
        connection: &PgConnection,
    ) -> Result<i32, diesel::result::Error> {
        let new_content_id = connection.transaction::<i32, diesel::result::Error, _>(|| {
            let chapter = chapters::table
                .find(new_content.chapter_id)
                .first::<Chapter>(connection)?;

            let contents_ids = Content::belonging_to(&chapter)
                .select(contents::id)
                .load::<i32>(connection)?;

            diesel::update(contents::table.filter(contents::id.eq(any(contents_ids))))
                .filter(contents::index.ge(&new_content.index))
                .set(contents::index.eq(contents::index + 1))
                .execute(connection)?;

            let new_content_id = diesel::insert_into(contents::table)
                .values(new_content)
                .returning(contents::id)
                .get_result(connection)?;

            Ok(new_content_id)
        })?;

        Ok(new_content_id)
    }

    // TODO - Bulk insert

    #[cfg(feature = "editable")]
    pub fn delete(
        id: i32,
        connection: &PgConnection,
    ) -> Result<TAPIResponse<()>, diesel::result::Error> {
        connection.transaction::<(), diesel::result::Error, _>(|| {
            let content = contents::table.find(id).first::<Content>(connection)?;

            let chapter = chapters::table
                .find(content.chapter_id)
                .load::<Chapter>(connection)?;

            let contents_ids: Vec<i32> = Content::belonging_to(&chapter)
                .select(contents::id)
                .load::<i32>(connection)?;

            diesel::update(contents::table.filter(contents::id.eq(any(contents_ids))))
                .filter(contents::index.gt(content.index))
                .set(contents::index.eq(contents::index - 1))
                .execute(connection)?;

            diesel::delete(contents::table.filter(contents::id.eq(id))).execute(connection)?;

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
        let contents = Self::belonging_to(chapter)
            .order_by(contents::index)
            .load::<Self>(connection)?;

        Ok(contents
            .into_iter()
            .map(|c| ContentRepresentation::from_model(c, None))
            .collect())
    }
}
