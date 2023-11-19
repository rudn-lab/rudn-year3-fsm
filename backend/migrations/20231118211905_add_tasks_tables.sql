-- Add migration script here
CREATE TABLE IF NOT EXISTS task_group (
    id INTEGER NOT NULL PRIMARY KEY,
    slug TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    legend TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS task (
    id INTEGER NOT NULL PRIMARY KEY,
    group_id INTEGER NOT NULL REFERENCES task_group(id),
    slug TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    legend TEXT NOT NULL,
    script TEXT NOT NULL,
    model_solution_json TEXT NOT NULL
);