use {
    super::{from_model::FromModel, tags::Tag},
    crate::{
        diesel::{
            pg::expression::dsl::any, BelongingToDsl, ExpressionMethods, PgConnection, QueryDsl,
            RunQueryDsl,
        },
        diesel_full_text_search::{plainto_tsquery, TsVectorExtensions},
        schema::{project_images, project_tags, projects, tags},
        types::project_category::ProjectCategory,
        API_URL,
    },
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

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

type ProjectColumns = (
    projects::id,
    projects::title,
    projects::image,
    projects::description,
    projects::features,
    projects::visit_link,
    projects::live_link,
    projects::download_link,
    projects::git,
    projects::category,
);

const PROJECT_COLUMNS: ProjectColumns = (
    projects::id,
    projects::title,
    projects::image,
    projects::description,
    projects::features,
    projects::visit_link,
    projects::live_link,
    projects::download_link,
    projects::git,
    projects::category,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectRepresentation {
    pub id: i32,
    pub title: String,
    pub image: String,
    pub description: String,
    pub features: String,
    pub visit_link: Option<String>,
    pub live_link: Option<String>,
    pub download_link: Option<String>,
    pub git: Option<String>,
    pub tags: Vec<Tag>,
    pub gallery: Vec<String>,
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

impl FromModel<Project> for ProjectRepresentation {
    fn from_model(project: Project, connection: Option<&PgConnection>) -> Self {
        Self {
            tags: project
                .tags(connection.unwrap())
                .expect("Error loading tags."),
            gallery: project
                .gallery(connection.unwrap())
                .expect("Error loading gallery."),
            id: project.id,
            category: project.category,
            title: project.title,
            image: API_URL.to_owned() + &project.image,
            description: project.description,
            features: project.features,
            visit_link: project.visit_link,
            live_link: project.live_link,
            download_link: project.download_link,
            git: project.git,
        }
    }
}

impl Project {
    fn tags(&self, connection: &PgConnection) -> Result<Vec<Tag>, diesel::result::Error> {
        let tags_ids = ProjectTag::belonging_to(self).select(project_tags::tag_id);
        let tags = tags::table
            .filter(tags::id.eq(any(tags_ids)))
            .load::<Tag>(connection)?;
        Ok(tags)
    }

    fn gallery(&self, connection: &PgConnection) -> Result<Vec<String>, diesel::result::Error> {
        let images = ProjectImage::belonging_to(self)
            .select(project_images::image)
            .load::<String>(connection)?;

        Ok(images
            .iter()
            .map(|image| API_URL.to_owned() + image)
            .collect())
    }

    pub fn find(
        id: i32,
        connection: &PgConnection,
    ) -> Result<ProjectRepresentation, diesel::result::Error> {
        let project = projects::table
            .select(PROJECT_COLUMNS)
            .find(id)
            .first::<Self>(connection)?;

        Ok(ProjectRepresentation::from_model(project, Some(connection)))
    }

    pub fn list(
        connection: &PgConnection,
    ) -> Result<HashMap<i32, ProjectRepresentation>, diesel::result::Error> {
        let projects = projects::table.select(PROJECT_COLUMNS).load(connection)?;

        let results: HashMap<i32, ProjectRepresentation> = projects
            .into_iter()
            .map(|project: Self| {
                (
                    project.id,
                    ProjectRepresentation::from_model(project, Some(connection)),
                )
            })
            .collect();

        Ok(results)
    }

    pub fn search(
        query: &str,
        connection: &PgConnection,
    ) -> Result<HashMap<i32, ProjectRepresentation>, diesel::result::Error> {
        let projects = projects::table
            .select(PROJECT_COLUMNS)
            .filter(projects::text_searchable_project.matches(plainto_tsquery(query)))
            // .limit(10)
            .load::<Self>(connection)?;
        let results: HashMap<i32, ProjectRepresentation> = projects
            .into_iter()
            .map(|project: Self| {
                (
                    project.id,
                    ProjectRepresentation::from_model(project, Some(connection)),
                )
            })
            .collect();

        Ok(results)
    }

    pub fn tagged(
        label: &str,
        connection: &PgConnection,
    ) -> Result<HashMap<i32, ProjectRepresentation>, diesel::result::Error> {
        let tag = tags::table
            .filter(tags::label.eq(label))
            .first::<Tag>(connection)?;

        let project_ids = ProjectTag::belonging_to(&tag)
            .select(project_tags::project_id)
            .load::<i32>(connection)?;
        let projects = projects::table
            .select(PROJECT_COLUMNS)
            .filter(projects::id.eq(any(project_ids)))
            .load::<Self>(connection)?;

        let results: HashMap<i32, ProjectRepresentation> = projects
            .into_iter()
            .map(|project: Self| {
                (
                    project.id,
                    ProjectRepresentation::from_model(project, Some(connection)),
                )
            })
            .collect();

        Ok(results)
    }

    #[cfg(feature = "editable")]
    pub fn resume_projects(
        connection: &PgConnection,
    ) -> Result<HashMap<i32, ProjectRepresentation>, diesel::result::Error> {
        // tinyDLM, guimauve.io, institut-sylvie.fr
        let resume_project_ids = vec![8, 2, 1];
        let resume_projects = projects::table
            .select(PROJECT_COLUMNS)
            .filter(projects::id.eq(any(resume_project_ids)))
            .load::<Self>(connection)?;

        let resume_projects_results: HashMap<i32, ProjectRepresentation> = resume_projects
            .into_iter()
            .map(|project: Self| {
                (
                    project.id,
                    ProjectRepresentation::from_model(project, Some(connection)),
                )
            })
            .collect();

        Ok(resume_projects_results)
    }
}
