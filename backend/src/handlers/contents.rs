#[cfg(feature = "editable")]
use {
    crate::{
        code::highlight_code,
        errors::database_error::DatabaseError,
        interfaces::InputContent,
        models::{
            articles::Article,
            contents::{Content, NewContent},
        },
        types::content_type::ContentType,
        Pool,
    },
    actix_web::{web, Error, HttpResponse},
};

#[cfg(feature = "editable")]
pub async fn delete(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Content::delete(&id, &connection))
        .await
        .map(|response| HttpResponse::Ok().json(response))
        .map_err(DatabaseError)?)
}

#[cfg(feature = "editable")]
pub async fn update(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    body: web::Json<Content>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(
        web::block(move || Content::update(&id, body.into_inner(), &connection))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(DatabaseError)?,
    )
}

#[cfg(feature = "editable")]
pub async fn add(
    pool: web::Data<Pool>,
    json_content: web::Json<InputContent>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let connection = pool.get().unwrap();
        let article_id = json_content.article_id;
        // TODO - InputContent.into_inner(NewContent)?
        Content::add(
            NewContent {
                article_id,
                chapter_id: json_content.chapter_id,
                content_type: json_content.content_type.clone(),
                content: &json_content.content,
                language: json_content.language.clone(),
                highlighted_code: match &json_content.content_type {
                    ContentType::Code => {
                        let language = json_content
                            .language
                            .as_ref()
                            .expect("Code content should specify a language!")
                            .to_string();
                        Some(highlight_code(&json_content.content, &language))
                    }
                    _ => None,
                },
                url: Some(json_content.url.as_deref().unwrap_or("")),
                index: json_content.index,
            },
            &connection,
        )?;
        Article::find(&article_id, &connection)
    })
    .await
    .map(|article| HttpResponse::Ok().json(article))
    .map_err(DatabaseError)?)
}
