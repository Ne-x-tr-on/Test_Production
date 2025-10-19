use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use std::{net::SocketAddr, sync::Arc};
use serde_json::json;

mod db;
mod models;
use db::create_connection;
use models::User;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let pool = Arc::new(create_connection().await);

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .with_state(pool);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Server running on port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello Nextron from Railway! 🚀" }))
}

async fn get_users(State(pool): State<Arc<PgPool>>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&*pool)
        .await
        .unwrap_or_default();

    Json(users)
}

async fn create_user(State(pool): State<Arc<PgPool>>, Json(payload): Json<User>) -> Json<User> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, name, email, created_at)
         VALUES ($1, $2, $3, NOW())
         RETURNING *"
    )
    .bind(payload.id)
    .bind(payload.name)
    .bind(payload.email)
    .fetch_one(&*pool)
    .await
    .unwrap();

    Json(user)
}
