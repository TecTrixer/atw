use serde::{Deserialize, Serialize};

// Struct which is being posted when calling /api/createQuestion
#[derive(Serialize, Deserialize)]
pub struct CreateQuestionDetails {
    question: String,
    ending_at: String,
    created_by: String,
}
