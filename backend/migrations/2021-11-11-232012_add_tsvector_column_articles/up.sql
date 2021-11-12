ALTER TABLE articles ADD COLUMN text_searchable_article tsvector NOT NULL DEFAULT '';
UPDATE
   articles 
SET
   text_searchable_article = to_tsvector('english', title || ' ' || headline);
CREATE INDEX textsearch_idx 
ON articles USING GIN (text_searchable_article);
CREATE TRIGGER tsvectorupdatearticles BEFORE INSERT 
OR 
UPDATE
   ON articles FOR EACH ROW EXECUTE PROCEDURE tsvector_update_trigger(text_searchable_article, 'pg_catalog.english', title, headline);
