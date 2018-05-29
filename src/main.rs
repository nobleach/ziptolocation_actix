extern crate actix;
extern crate actix_web;
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use dotenv::dotenv;


use actix::prelude::*;
use actix_web::{http, server, App, Json, Result, HttpRequest, HttpResponse};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Serialize)]
struct Health {
    status: String,
}

fn health(req: HttpRequest) -> Result<Json<Health>> {
    Ok(Json(Health{status: "OK".to_string()}))
}

fn main() {
    let sys = System::new("ziptolocation");

    // Get database connection
    let connection = establish_connection();

    // Start web server
    server::new(
        move || {
            App::new()
                .resource("/health", |r| r.f(health))
        })
        .bind("127.0.0.1:8088")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
