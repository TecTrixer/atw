use super::establish_connection;
use crate::diesel::prelude::*;
use crate::schema::questions::dsl::*;
use crate::structs::Question;
use anyhow;
use uuid::Uuid;

pub fn create_question_query(new_question: Question) -> Result<Uuid, anyhow::Error> {
    let connection = establish_connection();
    match diesel::insert_into(questions)
        .values(&new_question)
        .execute(&connection)
    {
        Ok(_) => Ok(new_question.id),
        Err(x) => Err(anyhow::Error::new(x)),
    }
}

pub fn vote_yes_query(identifier: Uuid) -> Result<(), anyhow::Error> {
    let connection = establish_connection();
    let current_question: Question = match questions
        .filter(id.eq(identifier))
        .load::<Question>(&connection)
    {
        Ok(quest_arr) => {
            if quest_arr.len() == 0 {
                return Err(anyhow::Error::msg("No question found"));
            }
            quest_arr[0].clone()
        }
        Err(x) => return Err(anyhow::Error::new(x)),
    };

    if !current_question.active {
        return Err(anyhow::Error::msg("This question is not active anymore!"));
    }

    match diesel::update(questions.filter(id.eq(identifier)))
        .set(votes_yes.eq(current_question.votes_yes + 1))
        .execute(&connection)
    {
        Ok(_) => Ok(()),
        Err(x) => Err(anyhow::Error::new(x)),
    }
}
pub fn vote_no_query(identifier: Uuid) -> Result<(), anyhow::Error> {
    let connection = establish_connection();
    let current_question: Question = match questions
        .filter(id.eq(identifier))
        .load::<Question>(&connection)
    {
        Ok(quest_arr) => {
            if quest_arr.len() == 0 {
                return Err(anyhow::Error::msg("No question found"));
            }
            quest_arr[0].clone()
        }
        Err(x) => return Err(anyhow::Error::new(x)),
    };

    if !current_question.active {
        return Err(anyhow::Error::msg("This question is not active anymore!"));
    }

    match diesel::update(questions.filter(id.eq(identifier)))
        .set(votes_no.eq(current_question.votes_no + 1))
        .execute(&connection)
    {
        Ok(_) => Ok(()),
        Err(x) => Err(anyhow::Error::new(x)),
    }
}
