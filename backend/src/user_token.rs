use crate::result::AppError;
use crate::AppState;
use api::{RegisterRequest, UserInfo, UserInfoResult};
use axum::{
    extract::{Path, State},
    Json,
};
use rand::{distributions::Alphanumeric, Rng};

pub async fn create_user(
    State(AppState { db }): State<AppState>,
    Json(RegisterRequest { name, rudn_id }): Json<RegisterRequest>,
) -> Result<Json<UserInfo>, AppError> {
    let mut token = String::with_capacity(16);
    {
        let mut rng = rand::thread_rng();
        for _ in 0..16 {
            token.push(rng.sample(Alphanumeric) as char);
        }
    }
    sqlx::query!(
        "INSERT INTO account (user_token, user_name, rudn_id) VALUES (?,?,?)",
        token,
        name,
        rudn_id
    )
    .execute(&db)
    .await?;

    Ok(Json(UserInfo {
        name,
        rudn_id,
        token,
    }))
}

pub async fn get_user(
    State(AppState { db }): State<AppState>,
    Path(token): Path<String>,
) -> Result<Json<UserInfoResult>, AppError> {
    let data = match sqlx::query!("SELECT * FROM account WHERE user_token=?", token)
        .fetch_optional(&db)
        .await?
    {
        Some(v) => UserInfoResult::Ok(UserInfo {
            name: v.user_name,
            rudn_id: v.rudn_id,
            token: v.user_token,
        }),
        None => UserInfoResult::NoSuchToken,
    };

    Ok(Json(data))
}
