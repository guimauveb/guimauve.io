use {
    crate::{
        errors::database_error::DatabaseError, interfaces::BlogQuery, service::search::db_search,
        Pool,
    },
    actix_web::{web, Error, HttpResponse},
};

pub async fn search(
    pool: web::Data<Pool>,
    query: web::Query<BlogQuery>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_search(pool, query.into_inner().text))
        .await
        .map(|results| HttpResponse::Ok().json(results))
        .map_err(DatabaseError)?)
}
