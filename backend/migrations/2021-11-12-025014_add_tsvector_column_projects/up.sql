ALTER TABLE projects ADD COLUMN text_searchable_project tsvector NOT NULL DEFAULT '';
UPDATE
   projects 
SET
   text_searchable_project = to_tsvector('english', title);
CREATE INDEX textsearch_project_idx 
ON projects USING GIN (text_searchable_project);
CREATE TRIGGER tsvectorupdateprojects BEFORE INSERT 
OR 
UPDATE
   ON projects FOR EACH ROW EXECUTE PROCEDURE tsvector_update_trigger(text_searchable_project, 'pg_catalog.english', title, headline);
