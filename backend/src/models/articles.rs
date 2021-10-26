/*
 * Order of structs must match the columns order!
 *  "When this trait is derived, it will assume that the order of fields on your struct match the order of the fields in the query.
 *   This means that field order is significant if you are using #[derive(Queryable)]. Field name has no effect."
*/
use {
    super::tags::Tag,
    crate::{
        schema::*,
        types::{content_type::ContentType, language::Language},
    },
    serde_derive::{Deserialize, Serialize},
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

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Clone, AsChangeset)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub preview: String,
    pub image: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "articles"]
pub struct NewArticleHeader<'a> {
    pub title: &'a str,
    pub preview: &'a str,
    pub published: bool,
    pub image: &'a str,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "chapters"]
pub struct NewChapter<'a> {
    pub article_id: i32,
    pub index: i32,
    pub title: &'a str,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "article_tags"]
pub struct NewArticleTag {
    pub article_id: i32,
    pub tag_id: i32,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct NewChapterForm<'a> {
    #[serde(borrow)]
    pub chapter: NewChapter<'a>,
    #[serde(borrow)]
    pub contents: Vec<NewContent<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewArticle<'a> {
    #[serde(borrow)]
    pub article_header: NewArticleHeader<'a>,
    pub tags: Vec<NewArticleTag>,
    #[serde(borrow)]
    pub chapters: Vec<NewChapterForm<'a>>,
}
