// モジュールの宣言
use crate::structs;
// インポート
use structs::health_check::{Register, Response};

pub fn create_response(value: Register) -> Response {
    let result = Response {
        id: 2,
        username: value.username,
        country: value.country
    };

    return result;
}