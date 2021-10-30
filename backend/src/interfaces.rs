use {
    crate::types::{
        content_type::ContentType, language::Language, project_category::ProjectCategory,
    },
    serde_derive::{Deserialize, Serialize},
    std::{collections::HashMap, fmt::Debug},
};

#[cfg(feature = "editable")]
#[derive(Debug, Serialize)]
pub enum Status {
    Success,
    _Error,
}

#[cfg(feature = "editable")]
#[derive(Debug, Serialize)]
pub struct TAPIResponse<T>
where
    T: Debug + 'static,
{
    pub status: Status,
    pub content: Option<T>,
}

#[derive(Debug, Serialize)]
pub struct SearchResults {
    pub articles: HashMap<i32, IArticle>,
    pub projects: HashMap<i32, IProject>,
}

#[derive(Debug, Serialize)]
pub struct ResultsIds {
    articles: Vec<i32>,
    projects: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ITag {
    pub id: i32,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IContent {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct IChapter {
    pub id: i32,
    pub article_id: i32,
    pub index: i32,
    pub title: String,
    pub contents: Vec<IContent>,
}

// TODO - Use it in IArticle
#[derive(Debug, Serialize, Deserialize)]
pub struct IArticleHeader {
    pub article_id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub tags: Vec<ITag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IArticle {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub tags: Vec<ITag>,
    pub chapters: Vec<IChapter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IProject {
    pub id: i32,
    pub title: String,
    pub image: String,
    pub description: String,
    pub features: String,
    pub visit_link: Option<String>,
    pub live_link: Option<String>,
    pub download_link: Option<String>,
    pub git: Option<String>,
    pub tags: Vec<ITag>,
    pub gallery: Option<Vec<String>>,
    pub category: ProjectCategory,
}

#[derive(Debug, Deserialize)]
pub struct InputTag {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
pub struct InputContent {
    pub chapter_id: i32,
    pub article_id: i32,
    pub index: i32,
    pub content_type: ContentType,
    pub content: String,
    pub language: Option<Language>,
    pub highlighted_code: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InputChapter {
    pub article_id: i32,
    pub index: i32,
    pub title: String,
    pub contents: Vec<InputContent>,
}

#[derive(Debug, Deserialize)]
pub struct InputArticle {
    pub title: String,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub tags: Vec<InputTag>,
    pub chapters: Vec<InputChapter>,
}

#[derive(Debug, Deserialize)]
pub struct InputPublishArticle {
    pub published: bool,
}

#[derive(Debug, Deserialize)]
pub struct BlogQuery {
    pub text: String,
}
