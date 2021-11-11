use {
    super::{articles::db_get_article_result_by_id, projects::db_get_project_result_by_id},
    crate::{
        errors::database_error::DatabaseError,
        interfaces::{IArticle, IProject, ITag, SearchResults},
        models::{articles::ArticleTag, projects::ProjectTag, tags::Tag},
        schema::{article_tags, articles, project_tags, tags},
        Pool, INCLUDE_UNPUBLISHED_ARTICLES,
    },
    actix_web::{web, Error, HttpResponse},
    diesel::{prelude::*, BelongingToDsl, QueryDsl, RunQueryDsl},
    std::collections::HashMap,
};

fn db_get_tags(pool: web::Data<Pool>) -> Result<Vec<ITag>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let all_tags = tags::table.load::<Tag>(&conn)?;
    let results = all_tags
        .into_iter()
        .map(|tag| ITag {
            id: tag.id,
            label: tag.label,
        })
        .collect();

    Ok(results)
}
pub async fn get_tags(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_tags(pool))
        .await
        .map(|tags| HttpResponse::Ok().json(tags))
        .map_err(|e| DatabaseError(e))?)
}

fn db_get_results_for_tag(
    pool: web::Data<Pool>,
    label: String,
) -> Result<SearchResults, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let conn = pool.get().unwrap();
    let tag = tags::table
        .filter(tags::label.eq(&label))
        .first::<Tag>(&conn)?;

    let articles_ids_for_tag = ArticleTag::belonging_to(&tag)
        .select(article_tags::article_id)
        .order_by(article_tags::id)
        .load::<i32>(&conn)
        .expect("Could not load article tags.");
    let articles_ids = match INCLUDE_UNPUBLISHED_ARTICLES {
        "true" => articles::table
            .filter(articles::id.eq(any(articles_ids_for_tag)))
            .select(articles::id)
            .order_by(articles::pub_date.desc())
            .load::<i32>(&conn)?,
        _ => articles::table
            .filter(articles::id.eq(any(articles_ids_for_tag)))
            .filter(articles::published.eq(true))
            .select(articles::id)
            .order_by(articles::pub_date.desc())
            .load::<i32>(&conn)?,
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

    let projects_ids = ProjectTag::belonging_to(&tag)
        .select(project_tags::project_id)
        .order_by(project_tags::id)
        .load::<i32>(&conn)
        .expect("Could not load project tags.");
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
pub async fn get_results_for_tag(
    pool: web::Data<Pool>,
    tag: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_results_for_tag(pool, tag.into_inner()))
            .await
            .map(|results| HttpResponse::Ok().json(results))
            .map_err(|e| DatabaseError(e))?,
    )
}
