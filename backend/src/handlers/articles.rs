use {
    crate::{
        interfaces::{IArticle, IChapter, IContent, ITag},
        models::{
            articles::{Article, ArticleTag, Chapter, Content},
            tags::Tag,
        },
        schema::{article_tags, articles, chapters, contents, tags},
        types::content_type::ContentType,
        Pool, API_URL, INCLUDE_UNPUBLISHED_ARTICLES,
    },
    actix_web::{web, Error, HttpResponse},
    diesel::{
        prelude::*,
        r2d2::{self, ConnectionManager},
        BelongingToDsl, QueryDsl, RunQueryDsl,
    },
    std::collections::HashMap,
};

#[cfg(feature = "editable")]
use crate::{
    code::highlight_code,
    interfaces::{
        IArticleHeader, InputArticle, InputChapter, InputContent, InputPublishArticle, Status,
        TAPIResponse,
    },
    models::articles::{
        NewArticle, NewArticleHeader, NewArticleTag, NewChapter, NewChapterForm, NewContent,
    },
    types::language::Language,
};

// Contents
fn db_get_contents_by_chapter(
    conn: &r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>,
    chapter: &Chapter,
) -> Result<Vec<Content>, diesel::result::Error> {
    let contents = Content::belonging_to(chapter)
        .order_by(contents::index)
        .load::<Content>(conn)
        .expect("Could not load contents.");

    Ok(contents)
}

fn db_get_contents_results_by_chapter(
    conn: &r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>,
    chapter: &Chapter,
) -> Result<Vec<IContent>, diesel::result::Error> {
    let contents = db_get_contents_by_chapter(conn, chapter)?;
    Ok(contents
        .into_iter()
        .map(move |content| IContent {
            id: content.id,
            article_id: content.article_id,
            chapter_id: content.chapter_id,
            index: content.index,
            content_type: content.content_type.clone(),
            content: match content.content_type {
                ContentType::Image => API_URL.to_owned() + &content.content,
                _ => content.content,
            },
            language: content.language,
            highlighted_code: content.highlighted_code,
            url: content.url,
        })
        .collect())
}
// TODO - Match content types.
#[cfg(feature = "editable")]
fn db_update_content(
    pool: web::Data<Pool>,
    pk: i32,
    body: Content,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let mut content = body;
    if content.content_type == ContentType::Code {
        let language = match &content.language {
            Some(language) => language.to_string(),
            None => Language::Bash.to_string(),
        };
        content.highlighted_code = Some(highlight_code(&content.content, &language));
    }

    diesel::update(contents::table.find(pk))
        .set(&content)
        .execute(&conn)?;

    db_get_article_result_by_id(pool, content.article_id)
}
#[cfg(feature = "editable")]
pub async fn update_content(
    pool: web::Data<Pool>,
    content_pk: web::Path<i32>,
    body: web::Json<Content>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_update_content(pool, content_pk.into_inner(), body.into_inner()))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[cfg(feature = "editable")]
fn db_add_content(
    pool: web::Data<Pool>,
    content: NewContent,
) -> Result<i32, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let conn = pool.get().unwrap();

    let content_id = conn.transaction::<i32, diesel::result::Error, _>(|| {
        let chapter = chapters::table
            .find(content.chapter_id)
            .first::<Chapter>(&conn)
            .expect("Could not load chapter.");

        let contents_ids = Content::belonging_to(&chapter)
            .select(contents::id)
            .load::<i32>(&conn)
            .expect("Could not load contents.");

        diesel::update(contents::table.filter(contents::id.eq(any(contents_ids))))
            .filter(contents::index.ge(content.index))
            .set(contents::index.eq(contents::index + 1))
            .execute(&conn)
            .expect("An error occured while incrementing contents ids.");

        let content_id = diesel::insert_into(contents::table)
            .values(&content)
            .get_result::<Content>(&conn)
            .expect("Could not insert content.")
            .id;

        Ok(content_id)
    })?;

    Ok(content_id)
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
                        let language = match &json_content.language {
                            Some(language) => &*language,
                            None => &Language::Bash,
                        };
                        Some(highlight_code(
                            &json_content.content,
                            &*language.to_string(),
                        ))
                    }
                    _ => None,
                },
                url: match &json_content.url {
                    Some(url) => Some(url),
                    None => None,
                },
                index: json_content.index,
            },
        )?;

        db_get_article_result_by_id(pool, article_id)
    })
    .await
    .map(|chapter| HttpResponse::Ok().json(chapter))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

#[cfg(feature = "editable")]
fn db_delete_content(
    pool: web::Data<Pool>,
    content_pk: i32,
) -> Result<TAPIResponse<()>, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let conn = pool.get().unwrap();
    conn.transaction::<(), diesel::result::Error, _>(|| {
        let content = contents::table
            .find(content_pk)
            .first::<Content>(&conn)
            .expect("Could not load chapter.");

        let chapter = chapters::table
            .find(content.chapter_id)
            .load::<Chapter>(&conn)
            .expect("Could not load chapter.");

        let contents_ids: Vec<i32> = Content::belonging_to(&chapter)
            .select(contents::id)
            .load::<i32>(&conn)
            .expect("Could not load contents ids.");

        diesel::update(contents::table.filter(contents::id.eq(any(contents_ids))))
            .filter(contents::index.gt(content.index))
            .set(contents::index.eq(contents::index - 1))
            .execute(&conn)
            .expect("An error occured while decrementing contents ids.");

        diesel::delete(contents::table.filter(contents::id.eq(content_pk)))
            .execute(&conn)
            .expect("Could not delete content.");

        Ok(())
    })?;

    Ok(TAPIResponse {
        status: Status::Success,
        content: Some(()),
    })
}
#[cfg(feature = "editable")]
pub async fn delete_content(
    pool: web::Data<Pool>,
    content_pk: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_delete_content(pool, content_pk.into_inner()))
            .await
            .map(|response| HttpResponse::Ok().json(response))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Chapters
fn db_get_chapters_by_article(
    conn: &r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>,
    article: &Article,
) -> Result<Vec<Chapter>, diesel::result::Error> {
    let chapters = Chapter::belonging_to(article)
        .order_by(chapters::index)
        .load::<Chapter>(conn)
        .expect("Could not load chapters.");

    Ok(chapters)
}
fn db_get_chapters_results_by_article(
    conn: &r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>,
    article: &Article,
) -> Result<Vec<IChapter>, diesel::result::Error> {
    let chapters = db_get_chapters_by_article(conn, article)?;

    Ok(chapters
        .into_iter()
        .map(move |chapter| IChapter {
            id: chapter.id,
            article_id: article.id,
            index: chapter.index,
            title: chapter.title.clone(),
            contents: match db_get_contents_results_by_chapter(conn, &chapter) {
                Ok(contents) => contents,
                Err(_) => vec![],
            },
        })
        .collect())
}

#[cfg(feature = "editable")]
fn db_update_chapter(
    pool: web::Data<Pool>,
    chapter_pk: i32,
    chapter: Chapter,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();

    diesel::update(chapters::table.find(chapter_pk))
        .set(&chapter)
        .execute(&conn)?;

    db_get_article_result_by_id(pool, chapter.article_id)
}
#[cfg(feature = "editable")]
pub async fn update_chapter(
    pool: web::Data<Pool>,
    chapter_pk: web::Path<i32>,
    body: web::Json<Chapter>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_update_chapter(pool, chapter_pk.into_inner(), body.into_inner()))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[cfg(feature = "editable")]
fn db_add_chapter(
    pool: web::Data<Pool>,
    chapter: NewChapter,
) -> Result<i32, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let conn = pool.get().unwrap();
    let chapter_id = conn.transaction::<i32, diesel::result::Error, _>(|| {
        let article = articles::table
            .find(chapter.article_id)
            .first::<Article>(&conn)
            .expect("Could not load article.");

        let chapters_ids = Chapter::belonging_to(&article)
            .select(chapters::id)
            .load::<i32>(&conn)
            .expect("Could not load chapters.");

        diesel::update(chapters::table.filter(chapters::id.eq(any(chapters_ids))))
            .filter(chapters::index.ge(chapter.index))
            .set(chapters::index.eq(chapters::index + 1))
            .execute(&conn)
            .expect("An error occured while incrementing chapters ids.");

        let chapter_id = diesel::insert_into(chapters::table)
            .values(&chapter)
            .get_result::<Chapter>(&conn)
            .expect("Could not insert chapter.")
            .id;

        Ok(chapter_id)
    })?;

    Ok(chapter_id)
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
    .map_err(|_| HttpResponse::InternalServerError())?)
}

#[cfg(feature = "editable")]
fn db_delete_chapter(
    pool: web::Data<Pool>,
    chapter_pk: i32,
) -> Result<TAPIResponse<()>, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let conn = pool.get().unwrap();
    conn.transaction::<(), diesel::result::Error, _>(|| {
        let chapter = chapters::table
            .find(chapter_pk)
            .first::<Chapter>(&conn)
            .expect("Could not load chapter.");

        let article = articles::table
            .find(chapter.article_id)
            .load::<Article>(&conn)
            .expect("Could not load article.");

        let chapters_ids: Vec<i32> = Chapter::belonging_to(&article)
            .select(chapters::id)
            .load::<i32>(&conn)
            .expect("An error occured while decrementing chapters ids.");

        diesel::update(chapters::table.filter(chapters::id.eq(any(chapters_ids))))
            .filter(chapters::index.gt(chapter.index))
            .set(chapters::index.eq(chapters::index - 1))
            .execute(&conn)
            .expect("Could not update chapter.");

        diesel::delete(chapters::table.filter(chapters::id.eq(chapter_pk))).execute(&conn)?;

        Ok(())
    })?;

    Ok(TAPIResponse {
        status: Status::Success,
        content: Some(()),
    })
}
#[cfg(feature = "editable")]
pub async fn delete_chapter(
    pool: web::Data<Pool>,
    chapter_pk: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_delete_chapter(pool, chapter_pk.into_inner()))
            .await
            .map(|response| HttpResponse::Ok().json(response))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Articles
fn db_get_tags_results_for_article(
    conn: &r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>,
    article: &Article,
) -> Result<Vec<ITag>, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let tags_ids = ArticleTag::belonging_to(article).select(article_tags::tag_id);
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
fn db_get_all_articles(pool: web::Data<Pool>) -> Result<Vec<Article>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let articles = match INCLUDE_UNPUBLISHED_ARTICLES {
        "true" => articles::table
            .order_by(articles::pub_date.desc())
            .load::<Article>(&conn)
            .expect("Could not load articles."),
        _ => articles::table
            .filter(articles::published.eq(true))
            .order_by(articles::pub_date.desc())
            .load::<Article>(&conn)
            .expect("Could not load articles."),
    };
    Ok(articles)
}
fn db_get_all_articles_results(
    pool: web::Data<Pool>,
) -> Result<HashMap<i32, IArticle>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let articles = db_get_all_articles(pool)?;
    let results: HashMap<i32, IArticle> = articles
        .into_iter()
        .map(|article: Article| {
            (
                article.id,
                IArticle {
                    tags: db_get_tags_results_for_article(&conn, &article)
                        .expect("Could not load article tags."),
                    chapters: db_get_chapters_results_by_article(&conn, &article)
                        .expect("Could not load chapters."),
                    id: article.id,
                    title: article.title,
                    pub_date: article.pub_date,
                    published: article.published,
                    headline: article.headline,
                    image: API_URL.to_owned() + &article.image,
                },
            )
        })
        .collect();

    Ok(results)
}

pub async fn get_all_articles(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_all_articles_results(pool))
        .await
        .map(|articles| HttpResponse::Ok().json(articles))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub fn db_get_article_result_by_id(
    pool: web::Data<Pool>,
    article_pk: i32,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let article = match INCLUDE_UNPUBLISHED_ARTICLES {
        "true" => articles::table
            .find(article_pk)
            .first::<Article>(&conn)
            .expect("Article not found."),
        _ => articles::table
            .filter(articles::published.eq(true))
            .find(article_pk)
            .first::<Article>(&conn)
            .expect("Article not found."),
    };
    let tags =
        db_get_tags_results_for_article(&conn, &article).expect("Could not load article tags.");
    let chapters = db_get_chapters_results_by_article(&conn, &article);

    // Might be a better way to do it -> Destructuring?
    Ok(IArticle {
        id: article.id,
        title: article.title,
        pub_date: article.pub_date,
        published: article.published,
        tags,
        headline: article.headline,
        image: API_URL.to_owned() + &article.image,
        chapters: match chapters {
            Ok(chapters) => chapters,
            Err(_) => vec![],
        },
    })
}
pub async fn get_article_by_id(
    pool: web::Data<Pool>,
    article_pk: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_article_result_by_id(pool, article_pk.into_inner()))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(|_| HttpResponse::NotFound())?,
    )
}

// Update article, chapters and contents are not supposed to be updated through this endpoint
#[cfg(feature = "editable")]
fn db_update_article_header(
    pool: web::Data<Pool>,
    pk: i32,
    header: IArticleHeader,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let article_header = Article {
        id: header.article_id,
        title: header.title,
        pub_date: header.pub_date,
        published: header.published,
        headline: header.headline,
        image: header.image,
    };

    diesel::update(articles::table.find(pk))
        .set(&article_header)
        .get_result::<Article>(&conn)
        .expect("Could not update article.");

    db_get_article_result_by_id(pool, pk)
}
#[cfg(feature = "editable")]
pub async fn update_article_header(
    pool: web::Data<Pool>,
    article_pk: web::Path<i32>,
    body: web::Json<IArticleHeader>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        db_update_article_header(pool, article_pk.into_inner(), body.into_inner())
    })
    .await
    .map(|article| HttpResponse::Ok().json(article))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

#[cfg(feature = "editable")]
fn db_add_article(
    pool: web::Data<Pool>,
    article: NewArticle,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let inserted_article: Article = diesel::insert_into(articles::table)
        .values(&article.article_header)
        .get_result(&conn)
        .expect("Could not insert article.");

    let inserted_article_id = inserted_article.id;

    for chap in article.chapters {
        let inserted_chapter_id = db_add_chapter(
            pool.clone(),
            NewChapter {
                article_id: inserted_article_id,
                ..chap.chapter
            },
        )?;
        for cont in chap.contents {
            db_add_content(
                pool.clone(),
                NewContent {
                    article_id: inserted_article_id,
                    chapter_id: inserted_chapter_id,
                    ..cont
                },
            )?;
        }
    }

    db_get_article_result_by_id(pool, inserted_article_id)
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
                            url: match &cont.url {
                                Some(url) => Some(url),
                                None => None,
                            },
                            language: match &cont.language {
                                Some(language) => Some(language.clone()),
                                None => None,
                            },
                        })
                        .collect(),
                })
                .collect(),
        };
        db_add_article(pool, new_article)
    })
    .await
    .map(|article| HttpResponse::Ok().json(article))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

#[cfg(feature = "editable")]
fn db_publish_article(
    pool: web::Data<Pool>,
    article_pk: i32,
    published: bool,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();
    diesel::update(articles::table.filter(articles::id.eq(article_pk)))
        .set(articles::published.eq(published))
        .execute(&conn)
        .expect("An error occured while updating the article.");

    db_get_article_result_by_id(pool, article_pk)
}
#[cfg(feature = "editable")]
pub async fn publish_article(
    pool: web::Data<Pool>,
    article_pk: web::Path<i32>,
    payload: web::Json<InputPublishArticle>,
) -> Result<HttpResponse, Error> {
    let published = payload.published;
    Ok(
        web::block(move || db_publish_article(pool, article_pk.into_inner(), published))
            .await
            .map(|response| HttpResponse::Ok().json(response))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[cfg(feature = "editable")]
fn db_delete_article(
    pool: web::Data<Pool>,
    article_pk: i32,
) -> Result<TAPIResponse<()>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    diesel::delete(articles::table.filter(articles::id.eq(article_pk))).execute(&conn)?;

    Ok(TAPIResponse {
        status: Status::Success,
        content: Some(()),
    })
}
#[cfg(feature = "editable")]
pub async fn delete_article(
    pool: web::Data<Pool>,
    article_pk: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_delete_article(pool, article_pk.into_inner()))
            .await
            .map(|response| HttpResponse::Ok().json(response))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// TODO - Frontend
#[cfg(feature = "editable")]
fn db_update_article_tags(
    pool: web::Data<Pool>,
    article_pk: i32,
    tags: Vec<Tag>,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let new_article_tags: Vec<NewArticleTag> = tags
        .into_iter()
        .map(|tag: Tag| NewArticleTag {
            article_id: article_pk,
            tag_id: tag.id,
        })
        .collect();

    conn.transaction::<(), _, _>(|| {
        diesel::delete(article_tags::table.filter(article_tags::article_id.eq(article_pk)))
            .execute(&conn)?;
        diesel::insert_into(article_tags::table)
            .values(new_article_tags)
            .execute(&conn)?;

        Err(diesel::result::Error::RollbackTransaction)
    })?;

    db_get_article_result_by_id(pool, article_pk)
}

#[cfg(feature = "editable")]
pub async fn update_article_tags(
    pool: web::Data<Pool>,
    article_pk: web::Path<i32>,
    body: web::Json<Vec<Tag>>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || {
            db_update_article_tags(pool, article_pk.into_inner(), body.into_inner())
        })
        .await
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
