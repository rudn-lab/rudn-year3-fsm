use api::{SubmissionVerdict, UserTaskSubmission};
use axum::{
    extract::{Path, State},
    Json,
};
use fsm::{
    fsm::StateMachine,
    tester::{FSMTester, FSMTestingOutput},
};

use crate::{result::AppError, AppState};

pub async fn submit_task(
    State(AppState { db }): State<AppState>,
    Path((_group_slug, task_slug, user_token)): Path<(String, String, String)>,
    Json(fsm): Json<StateMachine>,
) -> Result<Json<UserTaskSubmission>, AppError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let user_id = match sqlx::query!("SELECT * FROM account WHERE user_token=?", user_token)
        .fetch_optional(&db)
        .await?
    {
        Some(v) => v.id,
        None => return Err(anyhow::anyhow!("no such user to submit task to").into()),
    };

    let task = sqlx::query!("SELECT * FROM task WHERE slug=?", task_slug)
        .fetch_optional(&db)
        .await?;

    let task = if let Some(t) = task {
        t
    } else {
        return Err(anyhow::anyhow!("no such task to submit task to").into());
    };

    let seed = rand::random();
    let verdict = match testing_inner(&fsm, &task.script, seed) {
        Err(why) => SubmissionVerdict::TaskInternalError(format!("{why}")),
        Ok(v) => match v {
            FSMTestingOutput::Ok(tests) => SubmissionVerdict::Ok(tests),
            FSMTestingOutput::WrongAnswer {
                successes,
                total_tests,
                first_failure_seed,
                first_failure_expected_result,
            } => SubmissionVerdict::WrongAnswer {
                total_tests,
                successes,
                first_failure_seed,
                first_failure_expected_result,
            },
            FSMTestingOutput::FSMInvalid(validity) => SubmissionVerdict::InvalidFSM(validity),
        },
    };
    let is_ok = matches!(verdict, SubmissionVerdict::Ok(_));

    let fsm_json = serde_json::to_string(&fsm).unwrap();
    let verdict_json = serde_json::to_string(&verdict).unwrap();
    let rowid = sqlx::query!("INSERT INTO user_submission (when_unix_time, task_id, user_id, solution_json, init_random_seed, verdict_json, is_success) VALUES (?,?,?,?,?,?,?)",
        now,
        task.id,
        user_id,
        fsm_json,
        seed,
        verdict_json,
        is_ok
    ).execute(&db).await?.last_insert_rowid();

    Ok(Json(UserTaskSubmission {
        id: rowid,
        task_id: task.id,
        when_unix_time: now,
        solution: fsm,
        verdict,
    }))
}

fn testing_inner(fsm: &StateMachine, script: &str, seed: i64) -> anyhow::Result<FSMTestingOutput> {
    let mut tester = FSMTester::new(fsm.clone(), script)?;
    let res = tester.run_testing(seed)?;
    Ok(res)
}
