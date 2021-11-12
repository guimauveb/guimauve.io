table! {
    article_tags (id) {
        id -> Int4,
        article_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    articles (id) {
        id -> Int4,
        title -> Text,
        pub_date -> Timestamp,
        published -> Bool,
        headline -> Text,
        image -> Text,
        image_credits -> Nullable<Text>,
        text_searchable_article -> crate::diesel_full_text_search::TsVector,
    }

}

table! {
    chapters (id) {
        id -> Int4,
        article_id -> Int4,
        index -> Int4,
        title -> Text,
    }
}

table! {
    contents (id) {
        id -> Int4,
        article_id -> Int4,
        chapter_id -> Int4,
        index -> Int4,
        content_type -> crate::types::content_type::ContentTypeMapping,
        content -> Text,
        language -> Nullable<crate::types::language::LanguageMapping>,
        highlighted_code -> Nullable<Text>,
        url -> Nullable<Text>,
    }
}

table! {
    logs (id) {
        id -> Int4,
        created -> Timestamp,
        record_level -> Varchar,
        record -> Text,
    }
}

table! {
    project_images (id) {
        id -> Int4,
        project_id -> Int4,
        image -> Text,
    }
}

table! {
    project_tags (id) {
        id -> Int4,
        project_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    projects (id) {
        id -> Int4,
        title -> Varchar,
        image -> Text,
        description -> Text,
        features -> Text,
        visit_link -> Nullable<Varchar>,
        live_link -> Nullable<Varchar>,
        download_link -> Nullable<Varchar>,
        git -> Nullable<Varchar>,
        category -> crate::types::project_category::ProjectCategoryMapping,
        text_searchable_project -> crate::diesel_full_text_search::TsVector,
    }
}

table! {
    tags (id) {
        id -> Int4,
        label -> Text,
    }
}

joinable!(article_tags -> articles (article_id));
joinable!(article_tags -> tags (tag_id));
joinable!(chapters -> articles (article_id));
joinable!(contents -> articles (article_id));
joinable!(contents -> chapters (chapter_id));
joinable!(project_images -> projects (project_id));
joinable!(project_tags -> projects (project_id));
joinable!(project_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    article_tags,
    articles,
    chapters,
    contents,
    project_images,
    project_tags,
    projects,
    tags,
);
