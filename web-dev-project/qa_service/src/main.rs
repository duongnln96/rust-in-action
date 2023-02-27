use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use warp::{
    body::BodyDeserializeError, filters::cors::CorsForbidden, http::Method, http::StatusCode,
    reject::Reject, Filter, Rejection, Reply,
};

// === DTO ===
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
struct QuestionID(String);

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
struct AnswerID(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Question {
    id: QuestionID,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Answer {
    id: AnswerID,
    content: String,
    question_id: QuestionID,
}

// === Error handling ===

// Implement custom error
#[derive(Debug)]
enum Error {
    ParseInt(std::num::ParseIntError),
    MissingParameters,
    RangeInvalid,
    QuestionNotFound,
}

// 1. Implement the Display trait so Rust knows how to format the error to a string.
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseInt(ref error) => {
                write!(f, "Cannot parse parameter: {}", error)
            }
            Error::RangeInvalid => {
                write!(f, "Range invalid")
            }
            Error::MissingParameters => {
                write!(f, "Missing Parameter")
            }
            Error::QuestionNotFound => {
                write!(f, "Question not found")
            }
        }
    }
}

// 2. Implement Warp’s Reject trait on our error so we can return it in a Warp route handler.
impl Reject for Error {}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

// === Parsing ===
#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseInt)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseInt)?,
        });
    }

    Err(Error::MissingParameters)
}

// === In-Memmory store ===
#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionID, Question>>>,
    answers: Arc<RwLock<HashMap<AnswerID, Answer>>>,
}

impl Store {
    fn new() -> Self {
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

// === Handler ===
async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();

        let len_questions = res.len();

        // println!("len {}", len_questions);

        if pagination.end <= pagination.start || pagination.end > len_questions {
            return Err(warp::reject::custom(Error::RangeInvalid));
        }

        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();

        Ok(warp::reply::json(&res))
    }
}

async fn add_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionID(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

async fn delete_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionID(id)) {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer {
        id: AnswerID("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionID(params.get("questionId").unwrap().to_string()),
    };

    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}

// === Server ===
#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::POST, Method::DELETE, Method::GET]);

    // create a path Filter
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);

    let add_answer = warp::post()
        .and(warp::path("comments"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(add_answer);

    // routes define
    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        .recover(return_error);

    println!(" ==== Server is started ==== ");
    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
