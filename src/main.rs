#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Json;

#[get("/")]
fn index() -> Json<Vec<&'static str>> {
    Json(vec![
        "/api/v1/events",
        "/api/v1/meetups",
        "/api/v1/videos",
        "/api/v1/people",
    ])
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}