use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use serde_json::json;

use crate::core::code::ApiCode;

pub struct ApiResponse<T> {
    pub status: StatusCode,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn with_status(code: ApiCode, data: T) -> Self {
        ApiResponse {
            status: code.status(),
            message: code.message().to_string(),
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (self.status, Json(json!({"message": self.message, "data": self.data}))).into_response()
    }
}
