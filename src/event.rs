use actix_web::{HttpRequest, Json, Path, Responder};
use chrono::{serde::ts_seconds, DateTime, Utc};

#[derive(Debug, Serialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub is_published: bool,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>, // TODO: Option
    #[serde(with = "ts_seconds")]
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NewEvent {
    pub name: String,
    pub url: String,
    pub is_published: bool,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub started_at: DateTime<Utc>,
}

pub fn all(_req: HttpRequest) -> impl Responder {
    Json(vec![Event {
        id: 1,
        name: String::from("where"),
        url: String::from("http://pickfire.tk/"),
        is_published: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        started_at: Utc::now(),
    }])
}

pub fn create(event: Json<NewEvent>) -> impl Responder {
    info!("{:?}", &event);
    Json(json!({"success": true}))
}

pub fn update((params, _event): (Path<(i32,)>, Json<NewEvent>)) -> impl Responder {
    info!("{:?}", &params.0);
    Json(json!({"success": true}))
}

pub fn delete(params: Path<(i32,)>) -> impl Responder {
    info!("{:?}", &params.0);
    Json(json!({"success": true}))
}
