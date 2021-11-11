use {
    crate::{
        interfaces::{IArticle, IProject, SearchResults},
        schema::{articles, projects},
        service::{articles::db_get_article_result_by_id, projects::db_get_project_result_by_id},
        Pool, INCLUDE_UNPUBLISHED_ARTICLES,
    },
    actix_web::web,
    diesel::{prelude::*, QueryDsl, RunQueryDsl},
    std::collections::HashMap,
};

/* TODO
 * SELECT DISTINCT a.id,
 *                 a.pub_date,
 *                 a.title
 * FROM articles a
 * INNER JOIN article_tags atags ON a.id = atags.article_id
 * INNER JOIN tags t ON t.id = atags.tag_id
 * INNER JOIN CONTENTS con ON con.article_id = a.id
 * WHERE a.title ilike '%nginx%'
 *     OR t.label = 'nginx'
 *     OR con.content ilike '%nginx%'
 * ORDER BY a.id;
*/
// Remove format!
pub fn db_search(
    pool: web::Data<Pool>,
    query: String,
) -> Result<SearchResults, diesel::result::Error> {
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
