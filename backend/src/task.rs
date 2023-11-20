use std::collections::HashMap;

use api::{
    SmallTaskInfo, SubmissionVerdict, TaskGroupInfo, TaskInfo, UserTaskSubmission,
    UserTaskSubmissions,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

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
