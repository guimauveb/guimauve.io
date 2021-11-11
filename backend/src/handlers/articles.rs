use {
    crate::{
        errors::database_error::DatabaseError,
        service::articles::{db_get_all_articles_results, db_get_article_result_by_id},
        Pool,
    },
    actix_web::{web, Error, HttpResponse},
};

#[cfg(feature = "editable")]
use crate::{
    code::highlight_code,
    interfaces::{IArticleHeader, InputArticle, InputChapter, InputContent, InputPublishArticle},
    models::{
        articles::{
            Chapter, Content, NewArticle, NewArticleHeader, NewArticleTag, NewChapter,
            NewChapterForm, NewContent,
        },
        tags::Tag,
    },
    service::articles::{
        db_add_article, db_add_chapter, db_add_content, db_delete_article, db_delete_chapter,
        db_delete_content, db_publish_article, db_update_article_header, db_update_article_tags,
        db_update_chapter, db_update_content,
    },
    types::{content_type::ContentType, language::Language},
};

#[cfg(feature = "editable")]
pub async fn update_content(
    pool: web::Data<Pool>,
    content_id: web::Path<i32>,
    body: web::Json<Content>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_update_content(pool, content_id.into_inner(), body.into_inner()))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(DatabaseError)?,
    )
}

#[cfg(feature = "editable")]
pub async fn add_content(
    pool: web::Data<Pool>,
    json_content: web::Json<InputContent>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let article_id = json_content.article_id;
        db_add_content(
            pool.clone(),
            NewContent {
                article_id: json_content.article_id,
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
        )?;

        db_get_article_result_by_id(pool, article_id)
    })
    .await
    .map(|chapter| HttpResponse::Ok().json(chapter))
    .map_err(DatabaseError)?)
}

#[cfg(feature = "editable")]
pub async fn delete_content(
    pool: web::Data<Pool>,
    content_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_delete_content(pool, content_id.into_inner()))
            .await
            .map(|response| HttpResponse::Ok().json(response))
            .map_err(DatabaseError)?,
    )
}

#[cfg(feature = "editable")]
pub async fn update_chapter(
    pool: web::Data<Pool>,
    chapter_id: web::Path<i32>,
    body: web::Json<Chapter>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_update_chapter(pool, chapter_id.into_inner(), body.into_inner()))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(DatabaseError)?,
    )
}

#[cfg(feature = "editable")]
pub async fn add_chapter(
    pool: web::Data<Pool>,
    json_chapter: web::Json<InputChapter>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let article_id = json_chapter.article_id;
        db_add_chapter(
            pool.clone(),
            NewChapter {
                article_id,
                index: json_chapter.index,
                title: &json_chapter.title,
            },
        )?;
        db_get_article_result_by_id(pool, article_id)
    })
    .await
    .map(|article| HttpResponse::Ok().json(article))
    .map_err(DatabaseError)?)
}

#[cfg(feature = "editable")]
pub async fn delete_chapter(
    pool: web::Data<Pool>,
    chapter_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_delete_chapter(pool, chapter_id.into_inner()))
            .await
            .map(|response| HttpResponse::Ok().json(response))
            .map_err(DatabaseError)?,
    )
}

#[cfg(feature = "editable")]
pub async fn update_article_header(
    pool: web::Data<Pool>,
    article_id: web::Path<i32>,
    body: web::Json<IArticleHeader>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        db_update_article_header(pool, article_id.into_inner(), body.into_inner())
    })
    .await
    .map(|article| HttpResponse::Ok().json(article))
    .map_err(DatabaseError)?)
}

#[cfg(feature = "editable")]
pub async fn add_article(
    pool: web::Data<Pool>,
    json_article: web::Json<InputArticle>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let new_article = NewArticle {
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
        };
        db_add_article(pool, new_article)
    })
    .await
    .map(|article| HttpResponse::Ok().json(article))
    .map_err(DatabaseError)?)
}

#[cfg(feature = "editable")]
pub async fn publish_article(
    pool: web::Data<Pool>,
    article_id: web::Path<i32>,
    payload: web::Json<InputPublishArticle>,
) -> Result<HttpResponse, Error> {
    let published = payload.published;
    Ok(
        web::block(move || db_publish_article(pool, article_id.into_inner(), published))
            .await
            .map(|response| HttpResponse::Ok().json(response))
            .map_err(DatabaseError)?,
    )
}

#[cfg(feature = "editable")]
pub async fn delete_article(
    pool: web::Data<Pool>,
    article_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_delete_article(pool, article_id.into_inner()))
            .await
            .map(|response| HttpResponse::Ok().json(response))
            .map_err(DatabaseError)?,
    )
}

#[cfg(feature = "editable")]
pub async fn update_article_tags(
    pool: web::Data<Pool>,
    article_id: web::Path<i32>,
    body: web::Json<Vec<Tag>>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || {
            db_update_article_tags(pool, article_id.into_inner(), body.into_inner())
        })
        .await
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(DatabaseError)?,
    )
}

pub async fn get_all_articles(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_all_articles_results(pool))
        .await
        .map(|articles| HttpResponse::Ok().json(articles))
        .map_err(DatabaseError)?)
}

pub async fn get_article_by_id(
    pool: web::Data<Pool>,
    article_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_article_result_by_id(pool, article_id.into_inner()))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(DatabaseError)?,
    )
}
