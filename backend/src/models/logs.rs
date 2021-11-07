use {
    crate::schema::logs,
    serde_derive::{Deserialize, Serialize},
};

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct Log {
    pub id: i32,
    pub created: chrono::NaiveDateTime,
    pub record_level: String,
    pub record: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "logs"]
pub struct NewLog<'a> {
    pub record_level: &'a str,
    pub record: &'a str,
}
