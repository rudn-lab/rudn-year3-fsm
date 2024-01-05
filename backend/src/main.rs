mod others_submissions;
mod result;
pub mod submit;
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
        .route(
            "/tasks/:group/leaderboard",
            get(task::get_taskgroup_leaderboard),
        )
        .route("/task-by-id/:id", get(task::get_task_by_id))
        .route("/tasks/:group/:task", get(task::get_task))
        .route(
            "/tasks/:group/:task/:token",
            get(task::get_task_and_userdata).post(submit::submit_task),
        )
        .route(
            "/tasks/:group/:task/:token/success",
            get(task::get_task_success),
        )
        .route("/users", get(others_submissions::view_users))
        .route("/users/:id", get(others_submissions::view_specific_user))
        .route(
            "/submissions/:sid/:token",
            get(others_submissions::view_submission),
        )
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_methods(Any)
                .allow_origin("https://fsm.rudn-lab.ru".parse::<HeaderValue>().unwrap())
                //.allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
                .allow_headers(Any),
        )
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:5001".parse().unwrap())
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
        "GET /tasks/:group/:task -- get info about task\n",
        "GET /tasks/:group/:task/leaderboard -- get info about task's leaderboard\n",
        "GET /tasks/:group/:task/:token/success -- get whether the user has successfully solved this task\n",
        "GET /users -- get list of users and their cumulative stats\n",
        "GET /users/:userid -- get a particular user's submissions\n",
        "GET /submissions/:submissionid/:token -- get a particular submission, including its contents if the given user has also solved it",
        "GET /task-by-id/:task-id -- get info about a task by its ID",
    )
}
