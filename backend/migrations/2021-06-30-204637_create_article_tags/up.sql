CREATE TABLE IF NOT EXISTS article_tags
  (
     id                 SERIAL PRIMARY KEY,
     article_id         SERIAL REFERENCES articles(id)   ON DELETE CASCADE,
     tag_id             SERIAL REFERENCES tags(id)       ON DELETE CASCADE
  );
