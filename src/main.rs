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
use rocket_contrib::Json;

use events::Event;
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

#[get("/api/v1/events")]
fn read_events() -> Json<Vec<Event>> {
    Json(vec![Event::new("where", "http://pickfire.tk/", Utc::now())])
}

#[get("/api/v1/people")]
fn read_people() -> Json<Vec<People>> {
    Json(vec![People::new("foo", "bar", "panda.jpg")])
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, read_people, read_events])
        .launch();
}
