#![feature(try_trait_v2)]
mod result;
mod task;
mod user_token;

use axum::{
    http::HeaderValue,
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;
use tower_http::cors::Any;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url =
        std::env::var("DATABASE_URL").expect("No DATABASE_URL environment variable provided");

    let conn = sqlx::SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!().run(&conn).await?;

    let app_state = AppState { db: conn };

    let app = Router::new()
        .route("/", get(root))
        .route("/user-info", post(user_token::create_user))
        .route("/user-info/:token", get(user_token::get_user))
        .route("/tasks", get(task::get_taskgroups))
        .route("/tasks/:group", get(task::get_taskgroup))
        .route("/tasks/:group/:task", get(task::get_task))
        .route(
            "/tasks/:group/:task/:token",
            get(task::get_task_and_userdata),
        )
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_methods(Any)
                .allow_origin("https://fsm.rudn-lab.ru".parse::<HeaderValue>().unwrap())
                .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
                .allow_headers(Any),
        )
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    concat!(
        "Options:\n",
        "GET /user-info/:token -- get user info\n",
        "POST /user-info -- register and get new user's info\n",
        "GET /tasks -- get list of task groups\n",
        "GET /tasks/:group -- get list of tasks in a group\n",
        "GET /tasks/:group/:task -- get info about task\n"
    )
}
