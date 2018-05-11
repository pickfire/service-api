#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate chrono;

mod events;
mod people;

use chrono::Utc;
use rocket::{http::Status, response::Failure};
use rocket_contrib::Json;

use events::{Event, NewEvent};
use people::People;

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
    Json(vec![Event{
        id: 1,
        name: String::from("where"),
        url: String::from("http://pickfire.tk/"),
        is_published: false,
        create_at: Utc::now(),
        update_at: Utc::now(),
        start_at: Utc::now(),
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
fn read_people() -> Json<Vec<People>> {
    Json(vec![People {
        id: 1,
        name: String::from("foo"),
        about: String::from("bar"),
        profile: String::from("panda.jpg"),
    }])
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api/v1/people", routes![read_people])
        .mount("/api/v1/events", routes![read_events, create_event, update_event, delete_event])
        .launch();
}
