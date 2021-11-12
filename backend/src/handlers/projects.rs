use {
    crate::{errors::database_error::DatabaseError, models::projects::Project, Pool},
    actix_web::{web, Error, HttpResponse},
};

pub async fn list(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Project::list(&connection))
        .await
        .map(|projects| HttpResponse::Ok().json(projects))
        .map_err(DatabaseError)?)
}

#[cfg(feature = "editable")]
pub async fn get(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(
        web::block(move || Project::find(&connection, &id.into_inner()))
            .await
            .map(|project| HttpResponse::Ok().json(project))
            .map_err(DatabaseError)?,
    )
}
