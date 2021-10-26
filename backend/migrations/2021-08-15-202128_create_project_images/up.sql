CREATE TABLE IF NOT EXISTS project_images
  (
     id                 SERIAL PRIMARY KEY,
     project_id         SERIAL REFERENCES projects(id)   ON DELETE CASCADE,
     image              TEXT NOT NULL
  );
