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
    actix_web::web,
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
    interfaces::{IArticleHeader, Status, TAPIResponse},
    models::articles::{NewArticle, NewArticleTag, NewChapter, NewContent},
};

// Contents
#[cfg(feature = "editable")]
pub fn db_delete_content(
    pool: web::Data<Pool>,
    content_id: i32,
) -> Result<TAPIResponse<()>, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let conn = pool.get().unwrap();
    conn.transaction::<(), diesel::result::Error, _>(|| {
        let content = contents::table
            .find(content_id)
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

        diesel::delete(contents::table.filter(contents::id.eq(content_id)))
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
pub fn db_add_content(
    pool: web::Data<Pool>,
    new_content: NewContent,
) -> Result<i32, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let conn = pool.get().unwrap();

    let content_id = conn.transaction::<i32, diesel::result::Error, _>(|| {
        let chapter = chapters::table
            .find(new_content.chapter_id)
            .first::<Chapter>(&conn)
            .expect("Could not load chapter.");

        let contents_ids = Content::belonging_to(&chapter)
            .select(contents::id)
            .load::<i32>(&conn)
            .expect("Could not load contents.");

        diesel::update(contents::table.filter(contents::id.eq(any(contents_ids))))
            .filter(contents::index.ge(new_content.index))
            .set(contents::index.eq(contents::index + 1))
            .execute(&conn)
            .expect("An error occured while incrementing contents ids.");

        let content_id = diesel::insert_into(contents::table)
            .values(&new_content)
            .returning(contents::id)
            .get_result(&conn)
            .expect("Could not insert content.");

        Ok(content_id)
    })?;

    Ok(content_id)
}

#[cfg(feature = "editable")]
pub fn db_update_content(
    pool: web::Data<Pool>,
    pk: i32,
    updated_content: Content,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let mut content = updated_content;
    if content.content_type == ContentType::Code {
        let language = content
            .language
            .as_ref()
            .expect("Code content should specify a language!")
            .to_string();
        content.highlighted_code = Some(highlight_code(&content.content, &language));
    }

    diesel::update(contents::table.find(pk))
        .set(&content)
        .execute(&conn)?;

    db_get_article_result_by_id(pool, content.article_id)
}

pub fn db_get_contents_by_chapter(
    conn: &r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>,
    chapter: &Chapter,
) -> Result<Vec<Content>, diesel::result::Error> {
    let contents = Content::belonging_to(chapter)
        .order_by(contents::index)
        .load::<Content>(conn)
        .expect("Could not load contents.");

    Ok(contents)
}

pub fn db_get_contents_results_by_chapter(
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

// Chapters
#[cfg(feature = "editable")]
pub fn db_delete_chapter(
    pool: web::Data<Pool>,
    chapter_id: i32,
) -> Result<TAPIResponse<()>, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let conn = pool.get().unwrap();
    conn.transaction::<(), diesel::result::Error, _>(|| {
        let chapter = chapters::table
            .find(chapter_id)
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

        diesel::delete(chapters::table.filter(chapters::id.eq(chapter_id))).execute(&conn)?;

        Ok(())
    })?;

    Ok(TAPIResponse {
        status: Status::Success,
        content: Some(()),
    })
}

#[cfg(feature = "editable")]
pub fn db_add_chapter(
    pool: web::Data<Pool>,
    new_chapter: NewChapter,
) -> Result<i32, diesel::result::Error> {
    use diesel::pg::expression::dsl::any;
    let conn = pool.get().unwrap();
    let chapter_id = conn.transaction::<i32, diesel::result::Error, _>(|| {
        let article = articles::table
            .find(new_chapter.article_id)
            .first::<Article>(&conn)
            .expect("Could not load article.");

        let chapters_ids = Chapter::belonging_to(&article)
            .select(chapters::id)
            .load::<i32>(&conn)
            .expect("Could not load chapters.");

        diesel::update(chapters::table.filter(chapters::id.eq(any(chapters_ids))))
            .filter(chapters::index.ge(new_chapter.index))
            .set(chapters::index.eq(chapters::index + 1))
            .execute(&conn)
            .expect("An error occured while incrementing chapters ids.");

        let chapter_id = diesel::insert_into(chapters::table)
            .values(&new_chapter)
            .returning(chapters::id)
            .get_result(&conn)
            .expect("Could not insert chapter.");

        Ok(chapter_id)
    })?;

    Ok(chapter_id)
}

#[cfg(feature = "editable")]
pub fn db_update_chapter(
    pool: web::Data<Pool>,
    chapter_id: i32,
    updated_chapter: Chapter,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();

    diesel::update(chapters::table.find(chapter_id))
        .set(&updated_chapter)
        .execute(&conn)?;

    db_get_article_result_by_id(pool, updated_chapter.article_id)
}

pub fn db_get_chapters_results_by_article(
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

pub fn db_get_chapters_by_article(
    conn: &r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>,
    article: &Article,
) -> Result<Vec<Chapter>, diesel::result::Error> {
    let chapters = Chapter::belonging_to(article)
        .order_by(chapters::index)
        .load::<Chapter>(conn)
        .expect("Could not load chapters.");

    Ok(chapters)
}

// Articles
#[cfg(feature = "editable")]
pub fn db_delete_article(
    pool: web::Data<Pool>,
    article_id: i32,
) -> Result<TAPIResponse<()>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    diesel::delete(articles::table.filter(articles::id.eq(article_id))).execute(&conn)?;

    Ok(TAPIResponse {
        status: Status::Success,
        content: Some(()),
    })
}

#[cfg(feature = "editable")]
pub fn db_publish_article(
    pool: web::Data<Pool>,
    article_id: i32,
    published: bool,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();
    diesel::update(articles::table.filter(articles::id.eq(article_id)))
        .set(articles::published.eq(published))
        .execute(&conn)
        .expect("An error occured while updating the article.");

    db_get_article_result_by_id(pool, article_id)
}

#[cfg(feature = "editable")]
pub fn db_add_article(
    pool: web::Data<Pool>,
    new_article: NewArticle,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let inserted_article_id: i32 = diesel::insert_into(articles::table)
        .values(&new_article.article_header)
        .returning(articles::id)
        .get_result(&conn)
        .expect("Could not insert article.");

    for chapter_form in new_article.chapters {
        let inserted_chapter_id = db_add_chapter(
            pool.clone(),
            NewChapter {
                article_id: inserted_article_id,
                ..chapter_form.chapter
            },
        )?;
        for content in chapter_form.contents {
            db_add_content(
                pool.clone(),
                NewContent {
                    article_id: inserted_article_id,
                    chapter_id: inserted_chapter_id,
                    ..content
                },
            )?;
        }
    }

    db_get_article_result_by_id(pool, inserted_article_id)
}

#[cfg(feature = "editable")]
pub fn db_update_article_header(
    pool: web::Data<Pool>,
    pk: i32,
    updated_header: IArticleHeader,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();

    diesel::update(articles::table.find(pk))
        .set(&Article {
            id: updated_header.article_id,
            title: updated_header.title,
            pub_date: updated_header.pub_date,
            published: updated_header.published,
            headline: updated_header.headline,
            image: updated_header.image,
            image_credits: updated_header.image_credits,
        })
        .get_result::<Article>(&conn)
        .expect("Could not update article.");

    db_get_article_result_by_id(pool, pk)
}

pub fn db_get_article_result_by_id(
    pool: web::Data<Pool>,
    article_id: i32,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();
    println!("helo");
    let article = match INCLUDE_UNPUBLISHED_ARTICLES {
        "true" => articles::table
            .filter(articles::id.eq(article_id))
            .get_result::<Article>(&conn)?,
        _ => articles::table
            .filter(articles::published.eq(true))
            .filter(articles::id.eq(article_id))
            .get_result::<Article>(&conn)?,
    };
    let tags =
        db_get_tags_results_for_article(&conn, &article).expect("Could not load article tags.");
    let chapters = db_get_chapters_results_by_article(&conn, &article);

    Ok(IArticle {
        id: article.id,
        title: article.title,
        pub_date: article.pub_date,
        published: article.published,
        tags,
        headline: article.headline,
        image: API_URL.to_owned() + &article.image,
        image_credits: article.image_credits,
        chapters: match chapters {
            Ok(chapters) => chapters,
            Err(_) => vec![],
        },
    })
}

pub fn db_get_all_articles_results(
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
                    image_credits: article.image_credits,
                },
            )
        })
        .collect();

    Ok(results)
}

pub fn db_get_all_articles(pool: web::Data<Pool>) -> Result<Vec<Article>, diesel::result::Error> {
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

pub fn db_get_tags_results_for_article(
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

// TODO - Frontend
#[cfg(feature = "editable")]
pub fn db_update_article_tags(
    pool: web::Data<Pool>,
    article_id: i32,
    updated_tags: Vec<Tag>,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let new_article_tags: Vec<NewArticleTag> = updated_tags
        .into_iter()
        .map(|tag: Tag| NewArticleTag {
            article_id,
            tag_id: tag.id,
        })
        .collect();

    conn.transaction::<(), _, _>(|| {
        diesel::delete(article_tags::table.filter(article_tags::article_id.eq(article_id)))
            .execute(&conn)?;
        diesel::insert_into(article_tags::table)
            .values(new_article_tags)
            .execute(&conn)?;

        Err(diesel::result::Error::RollbackTransaction)
    })?;

    db_get_article_result_by_id(pool, article_id)
}
