use serde::{Deserialize, Serialize};

use crate::types::question::QuestionID;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct AnswerID(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerID,
    pub content: String,
    pub question_id: QuestionID,
}
