use {
    crate::{
        errors::database_error::DatabaseError, service::projects::db_get_all_projects_results, Pool,
    },
    actix_web::{web, Error, HttpResponse},
};

#[cfg(feature = "editable")]
use crate::service::projects::{db_get_project_result_by_id, db_get_resume_projects};

pub async fn get_all_projects(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_all_projects_results(pool))
        .await
        .map(|projects| HttpResponse::Ok().json(projects))
        .map_err(DatabaseError)?)
}

#[cfg(feature = "editable")]
pub async fn get_project_by_id(
    pool: web::Data<Pool>,
    project_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_project_result_by_id(pool, project_id.into_inner()))
            .await
            .map(|project| HttpResponse::Ok().json(project))
            .map_err(DatabaseError)?,
    )
}

#[cfg(feature = "editable")]
pub async fn get_resume_projects(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_resume_projects(pool))
        .await
        .map(|projects| HttpResponse::Ok().json(projects))
        .map_err(DatabaseError)?)
}
