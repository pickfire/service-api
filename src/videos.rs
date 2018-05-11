use chrono::{serde::ts_seconds, DateTime, Utc};

#[derive(Serialize)]
pub struct Video {
    pub meetup: i32,
    pub topic: String,
    pub speaker: Vec<i32>,

    #[serde(with = "ts_seconds")]
    pub create_at: DateTime<Utc>,
}
