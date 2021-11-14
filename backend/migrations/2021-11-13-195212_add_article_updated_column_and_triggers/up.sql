ALTER TABLE articles ADD COLUMN updated TIMESTAMP;

CREATE OR REPLACE FUNCTION update_article_update_column_on_article_update()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated = now();
    RETURN NEW;
END;
$$ language PLPGSQL;

CREATE TRIGGER update_article_updated_time_on_article_update BEFORE UPDATE OF title, headline, image, image_credits ON articles FOR EACH ROW EXECUTE PROCEDURE update_article_update_column_on_article_update();

CREATE OR REPLACE FUNCTION update_article_updated_column_on_chapter_or_content_update()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE articles
    SET updated = now()
    WHERE id = NEW.article_id;
    RETURN NULL;
END;
$$ language PLPGSQL;

CREATE TRIGGER update_article_updated_time_on_chapter_update AFTER INSERT OR UPDATE OR DELETE ON chapters FOR EACH ROW EXECUTE PROCEDURE update_article_updated_column_on_chapter_or_content_update();
CREATE TRIGGER update_article_updated_time_on_content_update AFTER INSERT OR UPDATE OR DELETE ON contents FOR EACH ROW EXECUTE PROCEDURE update_article_updated_column_on_chapter_or_content_update();
