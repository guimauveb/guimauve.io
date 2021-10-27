use {
    super::fetch::Fetch,
    crate::{
        entities::interfaces::{IProject, Status},
        API_URL,
    },
    std::collections::HashMap,
};

pub async fn get_all_projects() -> Result<HashMap<i32, IProject>, Status> {
    let url = format!("{}/projects", API_URL);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<HashMap<i32, IProject>>().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

#[cfg(feature = "editable")]
pub async fn get_resume_projects() -> Result<HashMap<i32, IProject>, Status> {
    let url = format!("{}/resume-projects", API_URL);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<HashMap<i32, IProject>>().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
