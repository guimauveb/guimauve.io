/* About Strings and concatenation (from https://rust-unofficial.github.io/patterns/idioms/concat-format.html)
 *      Using format! is usually the most succinct and readable way to combine strings, BUT:
 *      It is usually not the most efficient way to combine strings.
 *      A series of push operations on a mutable string is usually the most efficient (especially if the string has been pre-allocated to the expected size).
*/
use {
    super::fetch::Fetch,
    crate::{
        entities::interfaces::{ITag, SearchResults, Status},
        API_URL,
    },
};

pub async fn get_tag_list() -> Result<Vec<ITag>, Status> {
    // API_URL.len() + "/tags".len()
    let mut url = String::with_capacity(API_URL.len() + 5);
    url.push_str(API_URL);
    url.push_str("/tags");

    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(json.into_serde::<Vec<ITag>>().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn get_results_for_tag(tag: &str) -> Result<SearchResults, Status> {
    // API_URL.len() + "/tags/".len() + tag.len()
    let mut url = String::with_capacity(API_URL.len() + 6 + tag.len());
    url.push_str(API_URL);
    url.push_str("/tags/");
    url.push_str(&tag);

    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(json.into_serde::<SearchResults>().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
