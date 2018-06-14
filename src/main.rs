extern crate actix;
extern crate actix_web;

extern crate env_logger;
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate r2d2_postgres;

extern crate chrono;
extern crate listenfd;

mod event;
mod meetup;
mod user;
mod video;

use actix_web::{App, HttpRequest, HttpResponse, Json, http::Method, middleware, server};
use listenfd::ListenFd;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2_postgres::PostgresConnectionManager;

fn index(_req: HttpRequest) -> Json<[&'static str; 4]>{
    Json([
        "/api/v1/events",
        "/api/v1/meetups",
        "/api/v1/videos",
        "/api/v1/people",
    ])
}

fn main() {
    ::std::env::set_var("RUST_LOG", "service_api,actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // r2d2 pool
    let manager = if let Ok(url) = ::std::env::var("DATABASE_URL") {
        if url.starts_with("sqlite:") {
            SqliteConnectionManager::file(&url)
        } else {
            unimplemented!("Unsupported database URL: {}", url);
        }
    } else {
        panic!("DATABASE_URL is not defined");
    };
    let _pool = r2d2::Pool::new(manager).unwrap();

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(move || {
        App::new()
            .middleware(middleware::Logger::default())
            .route("/", Method::GET, index)
            .resource("/api/v1/events", |r| {
                r.method(Method::GET).with(event::all);
                r.method(Method::POST).with(event::create)
            })
            .resource("/api/v1/events/{id}", |r| {
                r.method(Method::PUT).with(event::update);
                r.method(Method::DELETE).with(event::delete)
            })
            .resource("/api/v1/meetups", |r| r.f(|_| HttpResponse::NotImplemented()))
            .resource("/api/v1/videos", |r| r.f(|_| HttpResponse::NotImplemented()))
            .resource("/api/v1/people", |r| r.f(|_| HttpResponse::NotImplemented()))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run();
}
