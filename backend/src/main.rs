use axum::{http::HeaderValue, routing::get, Router};
use sqlx::SqlitePool;
use tower_http::cors::Any;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
pub async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url =
        std::env::var("DATABASE_URL").expect("No DATABASE_URL environment variable provided");

    let conn = sqlx::SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let app_state = AppState { db: conn };

    let app = Router::new()
        .with_state(app_state)
        .route("/", get(root))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_methods(Any)
                .allow_origin("https://fsm.rudn-lab.ru".parse::<HeaderValue>().unwrap())
                .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
                .allow_headers(Any),
        );

    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    println!("req");
    "hello world"
}
