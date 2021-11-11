use {
    super::{articles::db_get_article_result_by_id, projects::db_get_project_result_by_id},
    crate::{
        errors::database_error::DatabaseError,
        interfaces::{BlogQuery, IArticle, IProject, SearchResults},
        schema::{articles, projects},
        Pool, INCLUDE_UNPUBLISHED_ARTICLES,
    },
    actix_web::{web, Error, HttpResponse},
    diesel::{prelude::*, QueryDsl, RunQueryDsl},
    std::collections::HashMap,
};

// TODO - Add columns
// Remove format!
fn db_search(pool: web::Data<Pool>, query: String) -> Result<SearchResults, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let articles_ids = match INCLUDE_UNPUBLISHED_ARTICLES {
        "true" => articles::table
            .filter(articles::title.ilike(format!("%{}%", &query)))
            .select(articles::id)
            .get_results::<i32>(&conn)?,
        _ => articles::table
            .filter(articles::title.ilike(format!("%{}%", &query)))
            .filter(articles::published.eq(true))
            .select(articles::id)
            .get_results::<i32>(&conn)?,
    };
    let articles: HashMap<i32, IArticle> = articles_ids
        .into_iter()
        .map(|id| {
            (id, {
                db_get_article_result_by_id(pool.clone(), id)
                    .expect("Error while loading article result.")
            })
        })
        .collect();

    let projects_ids = projects::table
        .filter(projects::title.ilike(format!("%{}%", &query)))
        .select(projects::id)
        .get_results::<i32>(&conn)?;
    let projects: HashMap<i32, IProject> = projects_ids
        .into_iter()
        .map(|id| {
            (id, {
                db_get_project_result_by_id(pool.clone(), id)
                    .expect("Error while loading project result.")
            })
        })
        .collect();

    Ok(SearchResults { articles, projects })
}
pub async fn search(
    pool: web::Data<Pool>,
    query: web::Query<BlogQuery>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_search(pool, query.into_inner().text))
        .await
        .map(|results| HttpResponse::Ok().json(results))
        .map_err(|err| DatabaseError(err))?)
}
