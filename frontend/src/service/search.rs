/* About Strings and concatenation (from https://rust-unofficial.github.io/patterns/idioms/concat-format.html)
 *      Using format! is usually the most succinct and readable way to combine strings, BUT:
 *      It is usually not the most efficient way to combine strings.
 *      A series of push operations on a mutable string is usually the most efficient (especially if the string has been pre-allocated to the expected size).
*/
use {
    super::fetch::Fetch,
    crate::{
        entities::interfaces::{SearchResults, Status},
        API_URL,
    },
};

pub async fn get_results_for_query(query: &str) -> Result<SearchResults, Status> {
    // API_URL.len() + "/search?text=".len() + query.len()
    let mut url = String::with_capacity(API_URL.len() + 13 + query.len());
    url.push_str(API_URL);
    url.push_str("/search?text=");
    url.push_str(query);

    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
