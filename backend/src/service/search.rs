use {
    crate::{
        interfaces::SearchResults,
        models::{articles::Article, projects::Project},
    },
    diesel::pg::PgConnection,
};

pub fn search(
    connection: &PgConnection,
    query: &str,
) -> Result<SearchResults, diesel::result::Error> {
    let articles = Article::search(query, connection)?;
    let projects = Project::search(query, connection)?;

    Ok(SearchResults { articles, projects })
}
