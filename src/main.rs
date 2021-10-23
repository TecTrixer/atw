mod handlers;
pub mod structs;
use actix_web::{web, App, HttpServer};
use handlers::{vote_no, vote_yes};
#[macro_use]
extern crate diesel_migrations;
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
// use diesel_migrations::embedded_migrations;

embed_migrations!();
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = establish_connection();
    embedded_migrations::run(&connection).expect("Expected migrations to run successfully :(");
    println!("Server started ...");
    HttpServer::new(|| {
        App::new()
            .route("/api/voteNo/{id}", web::get().to(vote_no))
            .route("/api/voteYes/{id}", web::get().to(vote_yes))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
