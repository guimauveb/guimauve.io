#[cfg(feature = "editable")]
use {
    crate::{
        errors::database_error::DatabaseError,
        interfaces::InputChapter,
        models::{
            articles::Article,
            chapters::{Chapter, NewChapter},
        },
        Pool,
    },
    actix_web::{web, Error, HttpResponse},
};

#[cfg(feature = "editable")]
pub async fn update(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    body: web::Json<Chapter>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Chapter::update(&connection, &id, &body))
        .await
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(DatabaseError)?)
}

#[cfg(feature = "editable")]
pub async fn add(
    pool: web::Data<Pool>,
    json_chapter: web::Json<InputChapter>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    let article_id = json_chapter.article_id;
    Ok(web::block(move || {
        Chapter::add(
            &connection,
            NewChapter {
                article_id,
                index: json_chapter.index,
                title: &json_chapter.title,
            },
        )?;
        Article::find(&connection, &article_id)
    })
    .await
    .map(|article| HttpResponse::Ok().json(article))
    .map_err(DatabaseError)?)
}

#[cfg(feature = "editable")]
pub async fn delete(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Chapter::delete(&connection, &id))
        .await
        .map(|response| HttpResponse::Ok().json(response))
        .map_err(DatabaseError)?)
}
