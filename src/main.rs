#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

mod people;

use rocket_contrib::Json;
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

#[get("/api/v1/people")]
fn people() -> Json<Vec<People>> {
    Json(vec![
        People::new("foo", "bar", "panda.jpg"),
    ])
}

fn main() {
    let routes = routes![index, people];
    rocket::ignite().mount("/", routes).launch();
}