CREATE TABLE IF NOT EXISTS project_tags
  (
     id                 SERIAL PRIMARY KEY,
     project_id         SERIAL REFERENCES projects(id)   ON DELETE CASCADE,
     tag_id             SERIAL REFERENCES tags(id)       ON DELETE CASCADE
  );
