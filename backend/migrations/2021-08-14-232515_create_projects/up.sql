DROP TYPE IF EXISTS project_category CASCADE;
CREATE TYPE project_category AS ENUM ('web_application', 'desktop_application');
CREATE TABLE IF NOT EXISTS projects
  (
     id                          SERIAL PRIMARY KEY,
     category                    project_category DEFAULT 'web_application',
     title                       VARCHAR(255) NOT NULL,
     image                       TEXT NOT NULL,
     description                 TEXT NOT NULL,
     features                    TEXT NOT NULL,
     visit_link                  VARCHAR(255) NULL,
     live_link                   VARCHAR(255) NULL,
     download_link               VARCHAR(255) NULL,
     git                         VARCHAR(255) NULL
  );
