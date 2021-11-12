use {
    crate::{
        errors::database_error::DatabaseError, interfaces::BlogQuery,
        service::search::search as search_service, Pool,
    },
    actix_web::{web, Error, HttpResponse},
};

pub async fn search(
    pool: web::Data<Pool>,
    query: web::Query<BlogQuery>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || search_service(&connection, &query.text))
        .await
        .map(|results| HttpResponse::Ok().json(results))
        .map_err(DatabaseError)?)
}
