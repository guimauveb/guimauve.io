use {
    super::{articles::Article, projects::Project},
    crate::{interfaces::TagResults, schema::tags},
    diesel::{PgConnection, RunQueryDsl},
    serde::{Deserialize, Serialize},
};

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Clone, AsChangeset)]
pub struct Tag {
    pub id: i32,
    pub label: String,
}

impl Tag {
    pub fn list(connection: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        let results = tags::table.load::<Self>(connection)?;
        Ok(results)
    }

    pub fn results(
        connection: &PgConnection,
        label: &str,
    ) -> Result<TagResults, diesel::result::Error> {
        let articles = Article::tagged(label, connection)?;
        let projects = Project::tagged(label, connection)?;

        Ok(TagResults { articles, projects })
    }
}
