use {
    super::fetch::Fetch,
    crate::{
        entities::interfaces::{
            IArticle, IArticleHeader, IChapter, IContent, IPublishArticle, Status, TAPIResponse,
        },
        API_URL,
    },
    serde_json::json,
    std::collections::HashMap,
};

pub async fn get_article_list() -> Result<HashMap<i32, IArticle>, Status> {
    let url = format!("{}/articles", API_URL);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<HashMap<i32, IArticle>>().unwrap()),

        Err(_err) => Err(Status::Error),
    }
}

pub async fn add_article(payload: &IArticle) -> Result<IArticle, Status> {
    let url = format!("{}/articles", API_URL);
    let json = Fetch::post(url, Some(json!(&payload).to_string())).await;

    match json {
        Ok(json) => Ok(json.into_serde::<IArticle>().unwrap()),

        Err(_err) => Err(Status::Error),
    }
}

pub async fn get_article(id: &i32) -> Result<IArticle, Status> {
    let url = format!("{}/articles/{}", API_URL, id);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<IArticle>().unwrap()),

        Err(_err) => Err(Status::Error),
    }
}

pub async fn update_article_header(payload: &IArticleHeader) -> Result<IArticle, Status> {
    let url = format!("{}/articles/{}", API_URL, &payload.article_id);
    let json = Fetch::patch(url, Some(json!(&payload).to_string())).await;

    match json {
        Ok(json) => Ok(json.into_serde::<IArticle>().unwrap()),

        Err(_err) => Err(Status::Error),
    }
}

pub async fn publish_article(id: &i32, payload: &IPublishArticle) -> Result<IArticle, Status> {
    let url = format!("{}/articles/publish/{}", API_URL, id);
    let json = Fetch::patch(url, Some(json!(&payload).to_string())).await;

    match json {
        Ok(json) => Ok(json.into_serde::<IArticle>().unwrap()),

        Err(_err) => Err(Status::Error),
    }
}

pub async fn delete_article(id: &i32) -> Result<Status, Status> {
    let url = format!("{}/articles/{}", API_URL, id);
    let json = Fetch::delete(url).await;

    match json {
        Ok(json) => {
            let response = json.into_serde::<TAPIResponse<()>>().unwrap();
            Ok(response.status)
        }
        Err(_err) => Err(Status::Error),
    }
}

pub async fn add_content(payload: &IContent) -> Result<IArticle, Status> {
    let url = format!("{}/contents", API_URL);
    let json = Fetch::post(url, Some(json!(&payload).to_string())).await;

    match json {
        Ok(json) => Ok(json.into_serde::<IArticle>().unwrap()),

        Err(_err) => Err(Status::Error),
    }
}

pub async fn update_content(payload: &IContent) -> Result<IArticle, Status> {
    let url = format!("{}/contents/{}", API_URL, &payload.id);
    let json = Fetch::patch(url, Some(json!(&payload).to_string())).await;

    match json {
        Ok(json) => Ok(json.into_serde::<IArticle>().unwrap()),

        Err(_err) => Err(Status::Error),
    }
}

pub async fn delete_content(id: &i32) -> Result<Status, Status> {
    let url = format!("{}/contents/{}", API_URL, id);
    let json = Fetch::delete(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<TAPIResponse<()>>().unwrap().status),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn add_chapter(payload: &IChapter) -> Result<IArticle, Status> {
    let url = format!("{}/chapters", API_URL);
    let json = Fetch::post(url, Some(json!(&payload).to_string())).await;

    match json {
        Ok(json) => Ok(json.into_serde::<IArticle>().unwrap()),

        Err(_err) => Err(Status::Error),
    }
}

pub async fn update_chapter(payload: &IChapter) -> Result<IArticle, Status> {
    let url = format!("{}/chapters/{}", API_URL, &payload.id);
    let json = Fetch::patch(url, Some(json!(&payload).to_string())).await;

    match json {
        Ok(json) => Ok(json.into_serde::<IArticle>().unwrap()),

        Err(_err) => Err(Status::Error),
    }
}

pub async fn delete_chapter(id: &i32) -> Result<Status, Status> {
    let url = format!("{}/chapters/{}", API_URL, id);
    let json = Fetch::delete(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<TAPIResponse<()>>().unwrap().status),
        Err(_err) => Err(Status::Error),
    }
}
