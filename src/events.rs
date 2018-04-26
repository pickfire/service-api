use chrono::{serde::ts_seconds, DateTime, Utc};

#[derive(Serialize)]
pub struct Event {
    pub id: Option<u64>,
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

impl Event {
    pub fn new(name: &str, url: &str, start_at: DateTime<Utc>) -> Self {
        Event {
            id: None,
            name: name.to_string(),
            url: url.to_string(),
            is_published: false,
            create_at: Utc::now(),
            update_at: Utc::now(),
            start_at: start_at,
        }
    }
}
