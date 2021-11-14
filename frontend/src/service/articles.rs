/* About Strings and concatenation (from https://rust-unofficial.github.io/patterns/idioms/concat-format.html)
 *      Using format! is usually the most succinct and readable way to combine strings, BUT:
 *      It is usually not the most efficient way to combine strings.
 *      A series of push operations on a mutable string is usually the most efficient (especially if the string has been pre-allocated to the expected size).
*/
use {
    super::fetch::Fetch,
    crate::{
        entities::interfaces::{IArticle, Status},
        API_URL,
    },
    std::collections::HashMap,
};

#[cfg(feature = "editable")]
use {
    crate::entities::interfaces::{
        IArticleHeader, IChapter, IContent, IPublishArticle, TAPIResponse,
    },
    serde_json::json,
};

pub async fn get_article_list() -> Result<HashMap<i32, IArticle>, Status> {
    // API_URL.len() + "/articles".len()
    let mut url = String::with_capacity(API_URL.len() + 9);
    url.push_str(API_URL);
    url.push_str("/articles");

    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn get_article(id: &i32) -> Result<IArticle, Status> {
    let id_str = id.to_string();
    // API_URL.len() + "/articles/".len() + id_str.len()
    let mut url = String::with_capacity(API_URL.len() + 10 + id_str.len());
    url.push_str(API_URL);
    url.push_str("/articles/");
    url.push_str(&id_str);

    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn add_article(payload: &IArticle) -> Result<IArticle, Status> {
    // API_URL.len() + "/articles".len()
    let mut url = String::with_capacity(API_URL.len() + 9);
    url.push_str(API_URL);
    url.push_str("/articles");

    let json = Fetch::post(url, Some(json!(&payload).to_string())).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn update_article_header(payload: &IArticleHeader) -> Result<IArticle, Status> {
    let article_id_str = &payload.id.to_string();
    // API_URL.len() + "/articles/".len() + article_id_str.len()
    let mut url = String::with_capacity(API_URL.len() + 10 + article_id_str.len());
    url.push_str(API_URL);
    url.push_str("/articles/");
    url.push_str(&article_id_str);

    let json = Fetch::patch(url, Some(json!(&payload).to_string())).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn publish_article(id: &i32, payload: &IPublishArticle) -> Result<IArticle, Status> {
    let id_str = id.to_string();
    // API_URL.len() + "/articles/publish/".len() + article_id_str.len()
    let mut url = String::with_capacity(API_URL.len() + 18 + id_str.len());
    url.push_str(API_URL);
    url.push_str("/articles/publish/");
    url.push_str(&id_str);

    let json = Fetch::patch(url, Some(json!(&payload).to_string())).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn delete_article(id: &i32) -> Result<Status, Status> {
    let id_str = id.to_string();
    // API_URL.len() + "/articles/".len() + id_str.len()
    let mut url = String::with_capacity(API_URL.len() + 10 + id_str.len());
    url.push_str(API_URL);
    url.push_str("/articles/");
    url.push_str(&id_str);

    let json = Fetch::delete(url).await;
    match json {
        Ok(json) => Ok(json.into_serde::<TAPIResponse<()>>().unwrap().status),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn add_content(payload: &IContent) -> Result<IArticle, Status> {
    // API_URL.len() + "/contents".len()
    let mut url = String::with_capacity(API_URL.len() + 9);
    url.push_str(API_URL);
    url.push_str("/contents");

    let json = Fetch::post(url, Some(json!(&payload).to_string())).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn update_content(payload: &IContent) -> Result<IArticle, Status> {
    let content_id_str = &payload.id.to_string();
    // API_URL.len() + "/contents/".len() + content_id_str.len()
    let mut url = String::with_capacity(API_URL.len() + 10 + content_id_str.len());
    url.push_str(API_URL);
    url.push_str("/contents/");
    url.push_str(&content_id_str);

    let json = Fetch::patch(url, Some(json!(&payload).to_string())).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn delete_content(id: &i32) -> Result<Status, Status> {
    let id_str = id.to_string();
    // API_URL.len() + "/contents/".len() + id_str.len()
    let mut url = String::with_capacity(API_URL.len() + 10 + id_str.len());
    url.push_str(API_URL);
    url.push_str("/contents/");
    url.push_str(&id_str);

    let json = Fetch::delete(url).await;
    match json {
        Ok(json) => Ok(json.into_serde::<TAPIResponse<()>>().unwrap().status),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn add_chapter(payload: &IChapter) -> Result<IArticle, Status> {
    // API_URL.len() + "/chapters".len()
    let mut url = String::with_capacity(API_URL.len() + 9);
    url.push_str(API_URL);
    url.push_str("/chapters");

    let json = Fetch::post(url, Some(json!(&payload).to_string())).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn update_chapter(payload: &IChapter) -> Result<IArticle, Status> {
    let id_str = payload.id.to_string();
    // API_URL.len() + "/chapters/".len() + id_str.len()
    let mut url = String::with_capacity(API_URL.len() + 10 + id_str.len());
    url.push_str(API_URL);
    url.push_str("/chapters/");
    url.push_str(&id_str);

    let json = Fetch::patch(url, Some(json!(&payload).to_string())).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn delete_chapter(id: &i32) -> Result<Status, Status> {
    let id_str = id.to_string();
    // API_URL.len() + "/chapters/".len() + id_str.len()
    let mut url = String::with_capacity(API_URL.len() + 10 + id_str.len());
    url.push_str(API_URL);
    url.push_str("/chapters/");
    url.push_str(&id_str);

    let json = Fetch::delete(url).await;
    match json {
        Ok(json) => Ok(json.into_serde::<TAPIResponse<()>>().unwrap().status),
        Err(_err) => Err(Status::Error),
    }
}
