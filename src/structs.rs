use crate::schema::*;
use serde::Deserialize;
use std::time::SystemTime;
use uuid::Uuid;

// Struct which is being posted when calling /api/createQuestion
#[derive(Clone, Debug, Queryable, Insertable)]
#[table_name = "questions"]
pub struct Question {
    pub id: Uuid,
    pub question: String,
    pub created_at: SystemTime,
    pub votes_yes: i32,
    pub votes_no: i32,
    pub created_by: String,
    pub active: bool,
}

#[derive(Deserialize)]
pub struct CreateQuestion {
    pub question: String,
    pub created_by: String,
}
