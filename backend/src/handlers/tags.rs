use {
    crate::{
        errors::database_error::DatabaseError,
        service::tags::{db_get_results_for_tag, db_get_tags},
        Pool,
    },
    actix_web::{web, Error, HttpResponse},
};

pub async fn get_tags(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_tags(pool))
        .await
        .map(|tags| HttpResponse::Ok().json(tags))
        .map_err(DatabaseError)?)
}

pub async fn get_results_for_tag(
    pool: web::Data<Pool>,
    tag: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_results_for_tag(pool, tag.into_inner()))
            .await
            .map(|results| HttpResponse::Ok().json(results))
            .map_err(DatabaseError)?,
    )
}
