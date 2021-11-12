ALTER TABLE projects DROP COLUMN text_searchable_project;
DROP TRIGGER tsvectorupdateprojects 
ON projects;
