CREATE TABLE IF NOT EXISTS logs
(
    id                                  SERIAL PRIMARY KEY,
    created                             TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    record_level                        varchar(5) NOT NULL,
    record                              TEXT NOT NULL
);
