#![allow(dead_code)]

use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    code: i32,
    msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        ApiResponse { code, msg, data }
    }
    pub fn ok(data: Option<T>) -> Self {
        ApiResponse::new(0, "ok".to_string(), data)
    }
    pub fn err<M: AsRef<str>>(code: i32, msg: M) -> Self {
        ApiResponse::new(code, String::from(msg.as_ref()), None)
    }
    pub fn err_msg<M: AsRef<str>>(msg: M) -> Self {
        ApiResponse::err(1, msg)
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
