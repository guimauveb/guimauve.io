use {
    crate::{
        interfaces::{IProject, ITag},
        models::{
            projects::{Project, ProjectImage, ProjectTag},
            tags::Tag,
        },
        schema::{project_images, project_tags, projects, tags},
        Pool, API_URL,
    },
    actix_web::{web, Error, HttpResponse},
    diesel::{
        prelude::*,
        r2d2::{self, ConnectionManager},
        BelongingToDsl, QueryDsl, RunQueryDsl,
    },
    std::collections::HashMap,
};

fn db_get_tags_results_for_project(
    conn: &r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>,
    project: &Project,
) -> Result<Vec<ITag>, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let tags_ids = ProjectTag::belonging_to(project).select(project_tags::tag_id);
    let tags = tags::table
        .filter(tags::id.eq(any(tags_ids)))
        .load::<Tag>(conn)
        .expect("Could not load tags.");
    let tags_results = tags
        .into_iter()
        .map(|t: Tag| ITag {
            id: t.id,
            label: t.label,
        })
        .collect::<Vec<ITag>>();

    Ok(tags_results)
}

fn db_get_project_gallery(
    conn: &r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>,
    project: &Project,
) -> Result<Vec<String>, diesel::result::Error> {
    let images = ProjectImage::belonging_to(project)
        .select(project_images::image)
        .order_by(project_images::id)
        .load::<String>(conn)
        .expect("Could not load project gallery.");

    Ok(images
        .iter()
        .map(|image| API_URL.to_owned() + image)
        .collect())
}

fn db_get_all_projects_results(
    pool: web::Data<Pool>,
) -> Result<HashMap<i32, IProject>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let projects = projects::table
        .order_by(projects::id)
        .load::<Project>(&conn)
        .expect("Cold not load projects.");

    let projects_results: HashMap<i32, IProject> = projects
        .into_iter()
        .map(|project: Project| {
            (
                project.id,
                IProject {
                    tags: db_get_tags_results_for_project(&conn, &project)
                        .expect("Could not load project tags."),
                    gallery: Some(
                        db_get_project_gallery(&conn, &project)
                            .expect("Could not load project gallery."),
                    ),
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
                },
            )
        })
        .collect();

    Ok(projects_results)
}
pub async fn get_all_projects(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_all_projects_results(pool))
        .await
        .map(|projects| HttpResponse::Ok().json(projects))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub fn db_get_project_result_by_id(
    pool: web::Data<Pool>,
    project_id: i32,
) -> Result<IProject, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let project = projects::table.find(project_id).first::<Project>(&conn)?;

    Ok(IProject {
        tags: db_get_tags_results_for_project(&conn, &project)
            .expect("Could not load project tags."),
        gallery: Some(
            db_get_project_gallery(&conn, &project).expect("Could not load project gallery."),
        ),
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
    })
}

#[cfg(feature = "editable")]
pub async fn get_project_by_id(
    pool: web::Data<Pool>,
    project_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_project_result_by_id(pool, project_id.into_inner()))
            .await
            .map(|project| HttpResponse::Ok().json(project))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[cfg(feature = "editable")]
fn db_get_resume_projects(
    pool: web::Data<Pool>,
) -> Result<HashMap<i32, IProject>, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let conn = pool.get().unwrap();

    // tinyDLM, guimauve.io, institut-sylvie.fr
    let resume_projects_ids = vec![8, 2, 1];
    let resume_projects = projects::table
        .filter(projects::id.eq(any(resume_projects_ids)))
        .load::<Project>(&conn)
        .expect("Could not load resume projects.");

    let resume_projects_results: HashMap<i32, IProject> = resume_projects
        .into_iter()
        .map(|project: Project| {
            (
                project.id,
                IProject {
                    tags: db_get_tags_results_for_project(&conn, &project)
                        .expect("Could not load project tags."),
                    gallery: Some(
                        db_get_project_gallery(&conn, &project)
                            .expect("Could not load project gallery."),
                    ),
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
                },
            )
        })
        .collect();

    Ok(resume_projects_results)
}

#[cfg(feature = "editable")]
pub async fn get_resume_projects(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_resume_projects(pool))
        .await
        .map(|projects| HttpResponse::Ok().json(projects))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
