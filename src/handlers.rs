// use crate::structs::Question;
use crate::queries::{create_question_query, vote_no_query, vote_yes_query};
use crate::structs::{CreateQuestion, Question};
use actix_web::{web, HttpResponse};
use serde_json;
use uuid::Uuid;

pub async fn vote_no(web::Path(id_str): web::Path<String>) -> HttpResponse {
    let id = match Uuid::parse_str(&id_str) {
        Ok(x) => x,
        Err(x) => return HttpResponse::BadRequest().json(x.to_string()),
    };
    match vote_no_query(id) {
        Ok(_) => HttpResponse::Ok().json(id.to_string()),
        Err(x) => HttpResponse::BadRequest().json(x.to_string()),
    }
}

pub async fn vote_yes(web::Path(id_str): web::Path<String>) -> HttpResponse {
    let id = match Uuid::parse_str(&id_str) {
        Ok(x) => x,
        Err(x) => return HttpResponse::BadRequest().json(x.to_string()),
    };
    match vote_yes_query(id) {
        Ok(_) => HttpResponse::Ok().json(id.to_string()),
        Err(x) => HttpResponse::BadRequest().json(x.to_string()),
    }
}

pub async fn create_question(input: String) -> HttpResponse {
    let question_input: CreateQuestion = match serde_json::from_str::<CreateQuestion>(&input) {
        Ok(x) => x,
        Err(x) => return HttpResponse::BadRequest().json(x.to_string()),
    };
    let question: Question = Question {
        id: Uuid::new_v4(),
        question: question_input.question,
        created_at: std::time::SystemTime::now(),
        votes_yes: 0,
        votes_no: 0,
        created_by: question_input.created_by,
        active: true,
    };
    match create_question_query(question) {
        Ok(x) => HttpResponse::Ok().json(x.to_string()),
        Err(x) => HttpResponse::InternalServerError().json(x.to_string()),
    }
}

pub async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().body("Hi there! I'm still working (at least for now) :)")
}
