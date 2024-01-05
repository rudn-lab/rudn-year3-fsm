use api::{
    OthersSubmissionDetails, OthersSubmissionInfo, SmallSubmissionInfo, SmallUserInfo,
    UserAndSubmissionStats, UserAndSubmissions,
};
use axum::{
    extract::{Path, State},
    Json,
};
use fsm::fsm::StateMachine;

use crate::{result::AppError, AppState};

pub async fn view_users(
    State(AppState { db }): State<AppState>,
) -> Result<Json<Vec<UserAndSubmissionStats>>, AppError> {
    let mut data = vec![];

    for row in sqlx::query!(r#"SELECT id, user_name, rudn_id, total_submissions, ok_submissions, attempted_tasks, ok_tasks FROM account
        INNER JOIN (SELECT user_id, count(1) total_submissions FROM user_submission GROUP BY user_id) t1 ON t1.user_id = account.id
        INNER JOIN (SELECT user_id, count(1) ok_submissions FROM user_submission WHERE is_success=1 GROUP BY user_id) t2 ON t2.user_id = account.id
        INNER JOIN (SELECT user_id, count(DISTINCT task_id) attempted_tasks FROM user_submission GROUP BY user_id) t3 ON t3.user_id = account.id
        INNER JOIN (SELECT user_id, count(DISTINCT task_id) ok_tasks FROM user_submission WHERE is_success=1 GROUP BY user_id) t4 ON t4.user_id = account.id
    "#).fetch_all(&db).await? {
        data.push(UserAndSubmissionStats {
            user: SmallUserInfo{
                id: row.id,
                name: row.user_name,
                rudn_id: row.rudn_id
            }, total_submissions: row.total_submissions as usize,
            ok_submissions: row.ok_submissions.unwrap_or(0) as usize,
            attempted_tasks: row.attempted_tasks.unwrap_or(0) as usize,
            ok_tasks: row.ok_tasks.unwrap_or(0) as usize,
        });
    }
    Ok(Json(data))
}

pub async fn view_specific_user(
    State(AppState { db }): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<UserAndSubmissions>, AppError> {
    let user = sqlx::query!(
        "SELECT id, user_name, rudn_id FROM account WHERE id=?",
        user_id
    )
    .fetch_optional(&db)
    .await?;
    if let Some(data) = user {
        let rows = sqlx::query!(
            "SELECT id, task_id, when_unix_time, verdict_json, solution_json FROM user_submission WHERE user_id=?",
            data.id
        )
        .fetch_all(&db)
        .await?;
        let mut submissions = vec![];
        for row in rows {
            let fsm: StateMachine = serde_json::from_str(&row.solution_json)?;
            submissions.push(SmallSubmissionInfo {
                id: row.id,
                task_id: row.task_id,
                when_unix_time: row.when_unix_time,
                verdict: serde_json::from_str(&row.verdict_json)?,
                node_count: fsm.nodes.len(),
                link_count: fsm.links.len(),
            });
        }
        Ok(Json(UserAndSubmissions::Present {
            user: SmallUserInfo {
                id: data.id,
                name: data.user_name,
                rudn_id: data.rudn_id,
            },
            submissions,
        }))
    } else {
        Ok(Json(UserAndSubmissions::UserNotFound))
    }
}

pub async fn view_submission(
    State(AppState { db }): State<AppState>,
    Path((sid, token)): Path<(i64, String)>,
) -> Result<Json<Option<OthersSubmissionInfo>>, AppError> {
    // Check whether this submission exists
    let submission_row = sqlx::query!("SELECT user_submission.id, when_unix_time, task_id, solution_json, verdict_json, account.id AS account_id, account.user_name AS account_name, account.rudn_id AS account_rudn_id FROM user_submission JOIN account ON account.id=user_submission.user_id WHERE user_submission.id=?", sid)
        .fetch_optional(&db)
        .await?;
    let submission_row = if let Some(v) = submission_row {
        v
    } else {
        return Ok(Json(None));
    };

    let mut info = OthersSubmissionInfo {
        id: submission_row.id,
        when_unix_time: submission_row.when_unix_time,
        task_id: submission_row.task_id,
        submitting_user: SmallUserInfo {
            id: submission_row.account_id,
            name: submission_row.account_name,
            rudn_id: submission_row.account_rudn_id,
        },
        verdict: serde_json::from_str(&submission_row.verdict_json)?,
        details: api::OthersSubmissionDetails::GuestAccess, // for now
    };

    // Check whether the user exists
    let user_row = sqlx::query!("SELECT * FROM account WHERE user_token=?", token)
        .fetch_optional(&db)
        .await?;
    let user_row = if let Some(v) = user_row {
        v
    } else {
        return Ok(Json(Some(info)));
    };

    info.details = OthersSubmissionDetails::SolveThisFirst;

    // Check whether the user has solved this task yet
    let my_user_submission = sqlx::query!(
        "SELECT id FROM user_submission WHERE task_id=? AND user_id=? AND is_success=1",
        submission_row.task_id,
        user_row.id
    )
    .fetch_optional(&db)
    .await?;
    if let Some(_) = my_user_submission {
        // User has solved this task already
        info.details =
            OthersSubmissionDetails::Ok(serde_json::from_str(&submission_row.solution_json)?);
    }

    Ok(Json(Some(info)))
}
