use {
    crate::{errors::database_error::DatabaseError, models::articles::Article, Pool},
    actix_web::{web, Error, HttpResponse},
};

#[cfg(feature = "editable")]
use crate::{
    code::highlight_code,
    interfaces::{InputArticle, InputPublishArticle},
    models::{
        articles::{NewArticle, NewArticleHeader, NewArticleTag},
        chapters::{NewChapter, NewChapterForm},
        contents::NewContent,
    },
    types::{content_type::ContentType, language::Language},
};

#[cfg(feature = "editable")]
pub async fn add(
    pool: web::Data<Pool>,
    json_article: web::Json<InputArticle>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || {
        // TODO - InputArticle.
        Article::add(
            &NewArticle {
                article_header: NewArticleHeader {
                    title: &json_article.title,
                    headline: &json_article.headline,
                    published: json_article.published,
                    image: &json_article.image,
                    image_credits: Some(json_article.image_credits.as_deref().unwrap_or("")),
                },
                tags: json_article
                    .tags
                    .iter()
                    .map(|t| NewArticleTag {
                        article_id: 0,
                        tag_id: t.id,
                    })
                    .collect(),
                chapters: json_article
                    .chapters
                    .iter()
                    .map(|chap| NewChapterForm {
                        chapter: NewChapter {
                            article_id: chap.article_id,
                            index: chap.index,
                            title: &chap.title,
                        },
                        contents: chap
                            .contents
                            .iter()
                            .map(|cont| NewContent {
                                article_id: cont.article_id,
                                chapter_id: cont.chapter_id,
                                index: cont.index,
                                content: &cont.content,
                                content_type: cont.content_type.clone(),
                                highlighted_code: match &cont.content_type {
                                    ContentType::Code => {
                                        let language = match &cont.language {
                                            Some(language) => &*language,
                                            None => &Language::Bash,
                                        };
                                        Some(highlight_code(&cont.content, &*language.to_string()))
                                    }
                                    _ => None,
                                },
                                url: Some(cont.url.as_deref().unwrap_or("")),
                                language: cont.language.clone(),
                            })
                            .collect(),
                    })
                    .collect(),
            },
            &connection,
        )
    })
    .await
    .map(|article| HttpResponse::Ok().json(article))
    .map_err(DatabaseError)?)
}

#[cfg(feature = "editable")]
pub async fn update(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    article: web::Json<Article>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(
        web::block(move || Article::update(*id, &article, &connection))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(DatabaseError)?,
    )
}

#[cfg(feature = "editable")]
pub async fn publish(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    payload: web::Json<InputPublishArticle>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    let published = payload.published;
    Ok(
        web::block(move || Article::publish(*id, published, &connection))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(DatabaseError)?,
    )
}

#[cfg(feature = "editable")]
pub async fn delete(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Article::delete(*id, &connection))
        .await
        .map(|response| HttpResponse::Ok().json(response))
        .map_err(DatabaseError)?)
}

pub async fn list(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Article::list(&connection))
        .await
        .map(|articles| HttpResponse::Ok().json(articles))
        .map_err(DatabaseError)?)
}

pub async fn get(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Article::get(*id, &connection))
        .await
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(DatabaseError)?)
}
