use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtoResponse<T> {
    pub item: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtoResponseMany<T> {
    pub items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtoId {
    pub id: i32,
}
