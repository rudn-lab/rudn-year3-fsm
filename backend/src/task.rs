use std::collections::HashMap;

use api::{
    SmallTaskInfo, SmallUserInfo, SubmissionVerdict, TaskGroupInfo, TaskGroupLeaderboard, TaskInfo,
    TaskLeaderboardRow, UserTaskSubmission, UserTaskSubmissions,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use fsm::fsm::StateMachine;

use crate::{result::AppError, AppState};

pub async fn get_taskgroups(
    State(AppState { db }): State<AppState>,
    // Path(token): Path<String>,
) -> Result<Json<Vec<TaskGroupInfo>>, AppError> {
    let mut task_groups: Vec<_> = sqlx::query!("SELECT * FROM task_group")
        .fetch_all(&db)
        .await?
        .into_iter()
        .map(|v| TaskGroupInfo {
            id: v.id,
            name: v.title,
            slug: v.slug,
            legend: v.legend,
            tasks: vec![],
        })
        .collect();

    {
        let mut task_lists = HashMap::new();
        for i in task_groups.iter_mut() {
            task_lists.insert(i.id, &mut i.tasks);
        }

        for task in sqlx::query!("SELECT * FROM task").fetch_all(&db).await? {
            task_lists
                .get_mut(&task.group_id)
                .unwrap()
                .push(SmallTaskInfo {
                    name: task.title,
                    slug: task.slug,
                });
        }
    }

    Ok(Json(task_groups))
}

pub async fn get_taskgroup(
    State(AppState { db }): State<AppState>,
    Path(slug): Path<String>,
) -> Result<(StatusCode, Json<Option<TaskGroupInfo>>), AppError> {
    let task_grp: Option<_> = sqlx::query!("SELECT * FROM task_group WHERE slug=?", slug)
        .fetch_optional(&db)
        .await?
        .map(|v| TaskGroupInfo {
            id: v.id,
            name: v.title,
            slug: v.slug,
            legend: v.legend,
            tasks: vec![],
        });

    if let Some(mut grp) = task_grp {
        for task in sqlx::query!("SELECT * FROM task WHERE group_id=?", grp.id)
            .fetch_all(&db)
            .await?
        {
            grp.tasks.push(SmallTaskInfo {
                name: task.title,
                slug: task.slug,
            });
        }
        Ok((StatusCode::OK, Json(Some(grp))))
    } else {
        Ok((StatusCode::NOT_FOUND, Json(None)))
    }
}

pub async fn get_taskgroup_leaderboard(
    State(AppState { db }): State<AppState>,
    Path(slug): Path<String>,
) -> Result<(StatusCode, Json<Option<TaskGroupLeaderboard>>), AppError> {
    let task_grp: Option<_> = sqlx::query!("SELECT * FROM task_group WHERE slug=?", slug)
        .fetch_optional(&db)
        .await?
        .map(|v| TaskGroupLeaderboard {
            id: v.id,
            name: v.title,
            slug: v.slug,
            legend: v.legend,
            tasks: vec![],
        });

    if let Some(mut grp) = task_grp {
        for task in sqlx::query!(
            "SELECT task.* FROM task
            JOIN task_group ON task.group_id=task_group.id
            WHERE task_group.id = ?",
            grp.id
        )
        .fetch_all(&db)
        .await?
        {
            let mut user_top_submissions = HashMap::new();
            for submission in sqlx::query!(
                "SELECT user_submission.* FROM user_submission WHERE task_id=?",
                task.id
            )
            .fetch_all(&db)
            .await?
            {
                let existing_by_user: Option<&mut (
                    SmallUserInfo,
                    i64,
                    i64,
                    usize,
                    usize,
                    SubmissionVerdict,
                )> = user_top_submissions.get_mut(&submission.user_id);

                let fsm: StateMachine = serde_json::from_str(&submission.solution_json)
                    .map_err(|v| anyhow::anyhow!("Invalid JSON in database: {v}"))?;
                let verdict: SubmissionVerdict = serde_json::from_str(&submission.verdict_json)
                    .map_err(|v| anyhow::anyhow!("Invalid JSON in database: {v}"))?;

                match existing_by_user {
                    Some(existing) => {
                        if existing.5.is_ok() && !verdict.is_ok() {
                            continue;
                        }
                        if existing.1 < submission.when_unix_time {
                            existing.1 = submission.when_unix_time;
                            existing.2 = submission.id;
                            existing.3 = fsm.nodes.len();
                            existing.4 = fsm.links.len();
                            existing.5 = verdict;
                        }
                    }
                    None => {
                        let user_info =
                            sqlx::query!("SELECT * FROM account WHERE id=?", submission.user_id)
                                .fetch_one(&db)
                                .await?;

                        user_top_submissions.insert(
                            submission.user_id,
                            (
                                SmallUserInfo {
                                    id: user_info.id,
                                    name: user_info.user_name,
                                    rudn_id: user_info.rudn_id,
                                },
                                submission.when_unix_time,
                                submission.id,
                                fsm.nodes.len(),
                                fsm.links.len(),
                                verdict,
                            ),
                        );
                    }
                }
            }

            grp.tasks.push(TaskLeaderboardRow {
                name: task.title,
                slug: task.slug,
                latest_submissions: user_top_submissions.into_values().collect(),
            })
        }
        Ok((StatusCode::OK, Json(Some(grp))))
    } else {
        Ok((StatusCode::NOT_FOUND, Json(None)))
    }
}

pub async fn get_task(
    State(AppState { db }): State<AppState>,
    Path((_group_slug, task_slug)): Path<(String, String)>,
) -> Result<(StatusCode, Json<Option<TaskInfo>>), AppError> {
    let task = sqlx::query!("SELECT * FROM task WHERE slug=?", task_slug)
        .fetch_optional(&db)
        .await?;

    if let Some(t) = task {
        Ok((
            StatusCode::OK,
            Json(Some(TaskInfo {
                name: t.title,
                slug: t.slug,
                legend: t.legend,
                script: t.script,
            })),
        ))
    } else {
        Ok((StatusCode::NOT_FOUND, Json(None)))
    }
}

pub async fn get_task_by_id(
    State(AppState { db }): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Option<TaskInfo>>), AppError> {
    let task = sqlx::query!("SELECT * FROM task WHERE id=?", id)
        .fetch_optional(&db)
        .await?;

    if let Some(t) = task {
        Ok((
            StatusCode::OK,
            Json(Some(TaskInfo {
                name: t.title,
                slug: t.slug,
                legend: t.legend,
                script: t.script,
            })),
        ))
    } else {
        Ok((StatusCode::NOT_FOUND, Json(None)))
    }
}

pub async fn get_task_and_userdata(
    State(AppState { db }): State<AppState>,
    Path((_group_slug, task_slug, user_token)): Path<(String, String, String)>,
) -> Result<(StatusCode, Json<Option<(TaskInfo, UserTaskSubmissions)>>), AppError> {
    let user_id = match sqlx::query!("SELECT * FROM account WHERE user_token=?", user_token)
        .fetch_optional(&db)
        .await?
    {
        Some(v) => Some(v.id),
        None => None,
    };

    let task = sqlx::query!("SELECT * FROM task WHERE slug=?", task_slug)
        .fetch_optional(&db)
        .await?;

    if let Some(t) = task {
        // Collect the user's submissions, if there is a user.
        let submissions = match user_id {
            Some(uid) => {
                let submissions: Vec<UserTaskSubmission> = sqlx::query!(
                    "SELECT * FROM user_submission WHERE task_id=? AND user_id=? ORDER BY when_unix_time DESC",
                    t.id,
                    uid
                )
                .fetch_all(&db)
                .await?
                .iter()
                .map(|v| UserTaskSubmission {
                    id: v.id,
                    task_id: v.task_id,
                    when_unix_time: v.when_unix_time,
                    solution: serde_json::from_str(&v.solution_json)
                        .expect("Invalid solution JSON found in database?"),
                    verdict: serde_json::from_str(&v.verdict_json)
                        .expect("Invalid verdict JSON found in database?"),
                })
                .collect();

                let latest_submission =
                    submissions.iter().max_by_key(|v| v.when_unix_time).cloned();
                let latest_ok_submission = submissions
                    .iter()
                    .filter(|v| matches!(v.verdict, SubmissionVerdict::Ok(_)))
                    .max_by_key(|v| v.when_unix_time)
                    .cloned();
                UserTaskSubmissions {
                    latest_submission,
                    latest_ok_submission,
                    submissions,
                }
            }
            None => UserTaskSubmissions::default(),
        };
        Ok((
            StatusCode::OK,
            Json(Some((
                TaskInfo {
                    name: t.title,
                    slug: t.slug,
                    legend: t.legend,
                    script: t.script,
                },
                submissions,
            ))),
        ))
    } else {
        Ok((StatusCode::NOT_FOUND, Json(None)))
    }
}

pub async fn get_task_success(
    State(AppState { db }): State<AppState>,
    Path((_group_slug, task_slug, user_token)): Path<(String, String, String)>,
) -> Result<(StatusCode, Json<bool>), AppError> {
    let user_id = match sqlx::query!("SELECT * FROM account WHERE user_token=?", user_token)
        .fetch_optional(&db)
        .await?
    {
        Some(v) => Some(v.id),
        None => None,
    };

    let task = sqlx::query!("SELECT * FROM task WHERE slug=?", task_slug)
        .fetch_optional(&db)
        .await?;

    if let Some(t) = task {
        // Collect the user's submissions, if there is a user.
        let success = match user_id {
            Some(uid) => sqlx::query!(
                "SELECT * FROM user_submission WHERE task_id=? AND user_id=? AND is_success=1",
                t.id,
                uid
            )
            .fetch_optional(&db)
            .await?
            .is_some(),
            None => false,
        };
        Ok((StatusCode::OK, Json(success)))
    } else {
        Ok((StatusCode::NOT_FOUND, Json(false)))
    }
}
