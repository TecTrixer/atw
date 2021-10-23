use crate::structs::CreateQuestionDetails;
use actix_web::{web, HttpResponse};

pub async fn vote_no(web::Path(id): web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello World: {}", id))
}

pub async fn vote_yes(web::Path(id): web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Goodbye World: {}", id))
}
