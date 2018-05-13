#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate chrono;

mod event;
mod meetup;
mod user;
mod video;

use chrono::Utc;
use rocket::{http::Status, response::Failure};
use rocket_contrib::Json;

use event::{Event, NewEvent};
use meetup::Meetup;
use user::User;
use video::Video;

#[get("/")]
fn index() -> Json<Vec<&'static str>> {
    Json(vec![
        "/api/v1/events",
        "/api/v1/meetups",
        "/api/v1/videos",
        "/api/v1/people",
    ])
}

#[get("/")]
fn read_events() -> Json<Vec<Event>> {
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

#[post("/", format = "application/json", data = "<event>")]
fn create_event(event: Json<NewEvent>) -> Failure {
    Failure(Status::NotImplemented)
}

#[put("/<id>", format = "application/json", data = "<event>")]
fn update_event(id: u64, event: Json<NewEvent>) -> Failure {
    Failure(Status::NotImplemented)
}

#[delete("/<id>")]
fn delete_event(id: u64) -> Failure {
    Failure(Status::NotImplemented)
}

#[get("/")]
fn read_meetups() -> Json<Vec<Meetup>> {
    Json(vec![Meetup {
        event: 1,
        organizer: vec![1],
    }])
}

#[get("/")]
fn read_people() -> Json<Vec<User>> {
    Json(vec![User {
        id: 1,
        name: String::from("foo"),
        about: String::from("bar"),
        profile: String::from("panda.jpg"),
        created_at: Utc::now(),
    }])
}

#[get("/")]
fn read_videos() -> Json<Vec<Video>> {
    Json(vec![Video {
        meetup: 1,
        topic: String::from("Hello world to Rust!"),
        speaker: vec![1],
        create_at: Utc::now(),
    }])
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount(
            "/api/v1/events",
            routes![read_events, create_event, update_event, delete_event],
        )
        .mount("/api/v1/meetups", routes![read_meetups])
        .mount("/api/v1/people", routes![read_people])
        .mount("/api/v1/videos", routes![read_videos])
        .launch();
}
