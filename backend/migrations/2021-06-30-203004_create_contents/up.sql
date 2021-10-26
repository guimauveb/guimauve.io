DROP TYPE IF EXISTS language CASCADE;
CREATE TYPE language AS ENUM ('rust', 'bash', 'python', 'sql', 'html', 'css', 'javascript', 'typescript', 'yaml');
DROP TYPE IF EXISTS content_type CASCADE;
CREATE TYPE content_type AS ENUM ('text', 'comment', 'link', 'code', 'image');
CREATE TABLE IF NOT EXISTS contents
  (
     id                 SERIAL PRIMARY KEY,
     article_id         SERIAL REFERENCES articles(id) ON DELETE CASCADE,
     chapter_id         SERIAL REFERENCES chapters(id) ON DELETE CASCADE,
     index              INT NOT NULL DEFAULT (0),
     content_type       content_type NOT NULL,
     content            TEXT NOT NULL,
     language           language,
     highlighted_code   TEXT NULL,
     url                TEXT NULL
  );

