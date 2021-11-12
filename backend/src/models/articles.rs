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

//impl ArticleList {
//    pub fn list(connection: &PgConnection, search: &str) -> Self {
//        use crate::schema;
//        use crate::schema::articles::dsl::*;
//        use diesel::pg::Pg;
//        use diesel::QueryDsl;
//        use diesel::RunQueryDsl;
//        use diesel_full_text_search::{plainto_tsquery, TsVectorExtensions};
//
//        let mut query = schema::articles::table.into_boxed::<Pg>();
//
//        if !search.is_empty() {
//            query = query.filter(text_searchable_article.matches(plainto_tsquery(search)));
//        }
//        let result = query
//            .select(ARTICLE_COLUMNS)
//            .limit(10)
//            .load::<Article>(connection)
//            .expect("Error loading articles");
//
//        ArticleList(result)
//    }
//}

//impl Article {
//    pub fn find(id: &i32, connection: &PgConnection) -> Result<Article, diesel::result::Error> {
//        use diesel::{QueryDsl, RunQueryDsl};
//
//        articles::table
//            .find(id)
//            .first(connection)
//    }
//
//    pub fn load() -> Result<Article, diesel::result::Error> {
//        articles::table
//            .find(id)
//            .select(ARTICLE_COLUMNS)
//            .first(connection)
//    }
//}
//
// Insertables
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
