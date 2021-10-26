use {
    super::{content_type::ContentType, language::Language, project_category::ProjectCategory},
    crate::{utils::date::get_current_readable_date, API_URL},
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

#[derive(Deserialize)]
pub enum Status {
    Success,
    Error,
    Unknown,
}

#[derive(Deserialize)]
pub struct TAPIResponse<T>
where
    T: 'static,
{
    pub status: Status,
    pub content: Option<T>,
}

#[derive(PartialEq, Deserialize, Clone)]
pub struct SearchResults {
    pub articles: HashMap<i32, IArticle>,
    pub projects: HashMap<i32, IProject>,
}

/* TODO - Store search results + query together
#[derive(Serialize, Deserialize)]
pub struct ISearch {
    pub query: String,
    pub results: SearchResults,
}*/

#[derive(Clone, PartialEq, Deserialize)]
pub struct ResultsIds {
    pub articles_ids: Vec<i32>,
    pub projects_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, Hash)]
pub struct ITag {
    pub id: i32,
    pub label: String,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct IContent {
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

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct IChapter {
    pub id: i32,
    pub article_id: i32,
    pub index: i32,
    pub title: String,
    pub contents: Vec<IContent>,
}

// TODO - Use it in IArticle (or not)
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct IArticleHeader {
    pub article_id: i32,
    pub title: String,
    pub pub_date: String,
    pub published: bool,
    pub preview: String,
    pub image: String,
    pub tags: Vec<ITag>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IArticle {
    pub id: i32,
    pub title: String,
    pub pub_date: String,
    pub published: bool,
    pub preview: String,
    pub image: String,
    pub tags: Vec<ITag>,
    pub chapters: Vec<IChapter>,
}

impl Default for IArticle {
    fn default() -> Self {
        IArticle {
            id: 0,
            title: "New article...".to_owned(),
            image: API_URL.to_owned() + "/media/images/articles/ferris.webp",
            preview: "Preview...".to_owned(),
            pub_date: get_current_readable_date().expect("Could not get current readable date."),
            published: false,
            chapters: vec![],
            tags: vec![],
        }
    }
}

#[derive(Serialize)]
pub struct IPublishArticle {
    pub published: bool,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
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
