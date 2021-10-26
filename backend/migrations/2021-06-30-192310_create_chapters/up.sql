CREATE TABLE IF NOT EXISTS chapters
  (
     id           SERIAL PRIMARY KEY,
     article_id   SERIAL REFERENCES articles(id) ON DELETE CASCADE,
     index        INT NOT NULL DEFAULT (0),
     title        TEXT NOT NULL
  );
