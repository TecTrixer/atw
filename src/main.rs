mod handlers;
pub mod structs;
use actix_web::{web, App, HttpServer};
use handlers::{vote_no, vote_yes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/voteNo/{id}", web::get().to(vote_no))
            .route("/api/voteYes/{id}", web::get().to(vote_yes))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
