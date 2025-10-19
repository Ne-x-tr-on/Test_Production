use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}
