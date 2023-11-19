CREATE TABLE IF NOT EXISTS user_submission (
    id INTEGER NOT NULL PRIMARY KEY,
    when_unix_time INTEGER NOT NULL,
    task_id INTEGER NOT NULL REFERENCES task(id),
    user_id INTEGER NOT NULL REFERENCES account(id),
    solution_json TEXT NOT NULL,
    init_random_seed INTEGER NOT NULL,
    verdict_json TEXT NOT NULL,
    is_success INTEGER NOT NULL
);

CREATE INDEX user_submission_when ON user_submission(when_unix_time);
CREATE INDEX user_submission_who ON user_submission(user_id);
CREATE INDEX user_submission_what ON user_submission(task_id);
