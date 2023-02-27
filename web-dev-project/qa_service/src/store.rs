use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::{
    answer::{Answer, AnswerID},
    question::{Question, QuestionID},
};
// === In-Memmory store ===
#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionID, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerID, Answer>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<QuestionID, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Cannot open questions.json")
    }
}
