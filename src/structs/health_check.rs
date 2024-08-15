use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct Register {
    pub username: String,
    pub country: String,
}

#[derive(Serialize)]
pub struct Response {
    pub id: i32,
    pub username: String,
    pub country: String,
}