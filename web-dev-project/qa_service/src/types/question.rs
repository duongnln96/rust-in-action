use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct QuestionID(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub id: QuestionID,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
