use sqlx::{postgres::PgPoolOptions, PgPool};
use dotenvy::dotenv;
use std::env;

/// Connects to PostgreSQL and runs pending migrations
pub async fn create_connection() -> PgPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("❌ Failed to connect to the database");

    // Run migrations (look inside /migrations)
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("❌ Failed to run database migrations");

    println!("✅ Database connected and migrations applied!");
    pool
}
