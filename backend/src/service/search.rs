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
    let articles = Article::search(connection, query)?;
    let projects = Project::search(connection, query)?;

    Ok(SearchResults { articles, projects })
}
