use std::collections::HashMap;

use api::{SmallTaskInfo, TaskGroupInfo, TaskInfo};
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
                testgen_script: t.testgen_script,
                testchk_script: t.testchk_script,
            })),
        ))
    } else {
        Ok((StatusCode::NOT_FOUND, Json(None)))
    }
}
