#[cfg(feature = "editable")]
use {
    crate::{errors::database_error::DatabaseError, models::projects::Project, Pool},
    actix_web::{web, Error, HttpResponse},
};

#[cfg(feature = "editable")]
pub async fn list(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Project::resume_projects(&connection))
        .await
        .map(|resume_projects| HttpResponse::Ok().json(resume_projects))
        .map_err(DatabaseError)?)
}
