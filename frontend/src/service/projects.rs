/* About Strings and concatenation (from https://rust-unofficial.github.io/patterns/idioms/concat-format.html)
 *      Using format! is usually the most succinct and readable way to combine strings, BUT:
 *      It is usually not the most efficient way to combine strings.
 *      A series of push operations on a mutable string is usually the most efficient (especially if the string has been pre-allocated to the expected size).
*/
use {
    super::fetch::Fetch,
    crate::{
        entities::interfaces::{IProject, Status},
        API_URL,
    },
    std::collections::HashMap,
};

pub async fn get_all_projects() -> Result<HashMap<i32, IProject>, Status> {
    // API_URL.len() + "/projects".len()
    let mut url = String::with_capacity(API_URL.len() + 9);
    url.push_str(API_URL);
    url.push_str("/projects");

    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn get_resume_projects() -> Result<HashMap<i32, IProject>, Status> {
    // API_URL.len() + "/resume-projects".len()
    let mut url = String::with_capacity(API_URL.len() + 16);
    url.push_str(API_URL);
    url.push_str("/resume-projects");

    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
