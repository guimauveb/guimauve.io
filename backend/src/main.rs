#[macro_use]
extern crate diesel;
extern crate diesel_full_text_search;
#[macro_use]
extern crate dotenv_codegen;
extern crate env_logger;
extern crate log;

use {
    actix_cors::Cors,
    actix_files as fs,
    actix_web::{http, middleware, web, App, HttpServer},
    diesel::r2d2::{self, ConnectionManager},
    diesel::PgConnection,
    logger::custom_logger::Logger,
};

mod code;
mod errors;
mod handlers;
mod interfaces;
mod logger;
mod models;
mod schema;
mod service;
mod types;

// Constants
const API_URL: &str = dotenv!("API_URL");
const DATABASE_URL: &str = dotenv!("DATABASE_URL");
const INCLUDE_UNPUBLISHED_ARTICLES: &str = dotenv!("INCLUDE_UNPUBLISHED_ARTICLES");

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("LOG_LEVEL", "info");

    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    Logger::init(pool.clone()).unwrap();

    HttpServer::new(move || {
        #[cfg(not(feature = "editable"))]
        return App::new()
            .wrap(middleware::Logger::new(
                "%r | %s | %a | %{Referer}i | %{X-Forwarded-For}i | %{User-Agent}i | %U | %D",
            ))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://192.168.1.12:3000")
                    .allowed_origin("https://guimauve.io")
                    .allowed_origin("https://www.guimauve.io")
                    .allowed_methods(vec!["GET", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(fs::Files::new("/media", "./media").show_files_listing())
            .data(pool.clone())
            .route(
                "/articles/{id}",
                web::get().to(handlers::articles::get_article_by_id),
            )
            .route(
                "/articles",
                web::get().to(handlers::articles::get_all_articles),
            )
            .route(
                "/tags/{tag}",
                web::get().to(handlers::tags::get_results_for_tag),
            )
            .route("/tags", web::get().to(handlers::tags::get_tags))
            .route("/search", web::get().to(handlers::search::search))
            .route(
                "/projects",
                web::get().to(handlers::projects::get_all_projects),
            );

        #[cfg(feature = "editable")]
        return App::new()
            .wrap(middleware::Logger::new(
                "%r | %s | %a | %{Referer}i | %{X-Forwarded-For}i | %{User-Agent}i | %U | %D",
            ))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://192.168.1.12:3000")
                    .allowed_methods(vec!["GET", "POST", "PATCH", "OPTIONS", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(fs::Files::new("/media", "./media").show_files_listing())
            .data(pool.clone())
            .service(
                web::resource("/articles")
                    .route(web::get().to(handlers::articles::get_all_articles))
                    .route(web::post().to(handlers::articles::add_article)),
            )
            .service(
                web::resource("/articles/{id}")
                    .route(web::get().to(handlers::articles::get_article_by_id))
                    .route(web::patch().to(handlers::articles::update_article_header))
                    .route(web::delete().to(handlers::articles::delete_article)),
            )
            .service(
                web::resource("/articles/{id}/tags")
                    .route(web::patch().to(handlers::articles::update_article_tags)),
            )
            .service(
                web::resource("/articles/publish/{id}")
                    .route(web::patch().to(handlers::articles::publish_article)),
            )
            .service(
                web::resource("/chapters").route(web::post().to(handlers::articles::add_chapter)),
            )
            .service(
                web::resource("/chapters/{id}")
                    .route(web::patch().to(handlers::articles::update_chapter))
                    .route(web::delete().to(handlers::articles::delete_chapter)),
            )
            .service(
                web::resource("/contents").route(web::post().to(handlers::articles::add_content)),
            )
            .service(
                web::resource("/contents/{id}")
                    .route(web::patch().to(handlers::articles::update_content))
                    .route(web::delete().to(handlers::articles::delete_content)),
            )
            .route(
                "/tags/{tag}",
                web::get().to(handlers::tags::get_results_for_tag),
            )
            .route("/tags", web::get().to(handlers::tags::get_tags))
            .route("/search", web::get().to(handlers::search::search))
            .route(
                "/projects",
                web::get().to(handlers::projects::get_all_projects),
            )
            .route(
                "/projects/{id}",
                web::get().to(handlers::projects::get_project_by_id),
            )
            .route(
                "/resume-projects",
                web::get().to(handlers::projects::get_resume_projects),
            );
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
