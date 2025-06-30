use axum::{Json, response::{IntoResponse, Response}};
use serde_json::json;

use crate::core::code::ApiCode;

pub struct ApiError {
    code: ApiCode,
}

impl ApiError {
    pub fn new(code: ApiCode) -> Self {
        Self { code }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.code.status(), Json(json!({"message": self.code.message()}))).into_response()
    }
}
