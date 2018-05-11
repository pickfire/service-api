use chrono::{serde::ts_seconds, DateTime, Utc};

#[derive(Serialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub is_published: bool,

    #[serde(with = "ts_seconds")]
    pub create_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub update_at: DateTime<Utc>, // TODO: Option
    #[serde(with = "ts_seconds")]
    pub start_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewEvent {
    pub name: String,
    pub url: String,
    pub is_published: bool,

    #[serde(with = "ts_seconds")]
    pub create_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub start_at: DateTime<Utc>,
}
