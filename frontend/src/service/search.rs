use {
    super::fetch::Fetch,
    crate::{
        entities::interfaces::{SearchResults, Status},
        API_URL,
    },
};

pub async fn get_results_for_query(query: &str) -> Result<SearchResults, Status> {
    let url = format!("{}/search?text={}", API_URL, query);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<SearchResults>().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
