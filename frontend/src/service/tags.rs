use {
    super::fetch::Fetch,
    crate::{
        entities::interfaces::{ITag, SearchResults, Status},
        API_URL,
    },
};

pub async fn get_tag_list() -> Result<Vec<ITag>, Status> {
    let url = format!("{}/tags", API_URL);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<Vec<ITag>>().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn get_results_for_tag(label: &str) -> Result<SearchResults, Status> {
    let url = format!("{}/tags/{}", API_URL, label);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<SearchResults>().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
