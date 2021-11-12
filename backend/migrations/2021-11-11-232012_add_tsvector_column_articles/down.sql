ALTER TABLE articles DROP COLUMN text_searchable_article;
DROP TRIGGER tsvectorupdatearticles 
ON articles;
