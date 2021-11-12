use {
    crate::{errors::database_error::DatabaseError, models::tags::Tag, Pool},
    actix_web::{web, Error, HttpResponse},
};

pub async fn list(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Tag::list(&connection))
        .await
        .map(|tags| HttpResponse::Ok().json(tags))
        .map_err(DatabaseError)?)
}

pub async fn get_results_for_tag(
    pool: web::Data<Pool>,
    tag: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Tag::results(&connection, &tag))
        .await
        .map(|results| HttpResponse::Ok().json(results))
        .map_err(DatabaseError)?)
}
