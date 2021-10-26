/*
 * Order of structs must match the columns order!
 *  "When this trait is derived, it will assume that the order of fields on your struct match the order of the fields in the query.
 *   This means that field order is significant if you are using #[derive(Queryable)]. Field name has no effect."
*/
use {
    super::tags::Tag,
    crate::{
        schema::{project_images, project_tags, projects},
        types::project_category::ProjectCategory,
    },
    serde_derive::{Deserialize, Serialize},
};

// Projects
#[derive(
    Identifiable, Debug, Associations, Serialize, Deserialize, Queryable, Clone, AsChangeset,
)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub image: String,
    pub description: String,
    pub features: String,
    pub visit_link: Option<String>,
    pub live_link: Option<String>,
    pub download_link: Option<String>,
    pub git: Option<String>,
    pub category: ProjectCategory,
}

#[derive(
    Debug, Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, AsChangeset,
)]
#[belongs_to(Project)]
pub struct ProjectImage {
    pub id: i32,
    pub project_id: i32,
    pub image: String,
}

#[derive(
    Debug, Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, AsChangeset,
)]
#[belongs_to(Project)]
#[belongs_to(Tag)]
pub struct ProjectTag {
    pub id: i32,
    pub project_id: i32,
    pub tag_id: i32,
}
