ALTER TABLE articles
  DROP COLUMN updated;

DROP TRIGGER update_article_updated_time_on_article_update ON articles;
DROP TRIGGER update_article_updated_time_on_chapter_update ON chapters;
DROP TRIGGER update_article_updated_time_on_content_update ON contents;

DROP FUNCTION update_article_update_column_on_article_update;
DROP FUNCTION update_article_updated_column_on_chapter_or_content_update;
