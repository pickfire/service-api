use chrono::{serde::ts_seconds, DateTime, Utc};

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub about: String,
    pub profile: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}
