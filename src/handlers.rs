use crate::queries::{
    create_question_query, get_question_query, get_result_query, vote_no_query, vote_yes_query,
};
use crate::structs::{CreateQuestion, GetQuestion, Question, ResultQuestion};
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

pub async fn get_question() -> HttpResponse {
    let quest: Question = match get_question_query() {
        Ok(x) => x,
        Err(x) => return HttpResponse::InternalServerError().json(x.to_string()),
    };
    let res: GetQuestion = GetQuestion {
        id: quest.id,
        question: quest.question,
        created_by: quest.created_by,
    };
    HttpResponse::Ok().json(res)
}

pub async fn get_result(web::Path(id_str): web::Path<String>) -> HttpResponse {
    let id = match Uuid::parse_str(&id_str) {
        Ok(x) => x,
        Err(x) => return HttpResponse::BadRequest().json(x.to_string()),
    };
    let quest: Question = match get_result_query(id) {
        Ok(x) => x,
        Err(x) => return HttpResponse::BadRequest().json(x.to_string()),
    };
    let res_question: ResultQuestion = ResultQuestion {
        question: quest.question,
        votes_yes: quest.votes_yes,
        votes_no: quest.votes_no,
        created_by: quest.created_by,
        created_at: quest
            .created_at
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };
    HttpResponse::Ok().json(res_question)
}

pub async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().body("Hi there! I'm still working (at least for now) :)")
}
