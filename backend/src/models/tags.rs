use {
    crate::schema::tags,
    serde_derive::{Deserialize, Serialize},
};

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Clone, AsChangeset)]
pub struct Tag {
    pub id: i32,
    pub label: String,
}
