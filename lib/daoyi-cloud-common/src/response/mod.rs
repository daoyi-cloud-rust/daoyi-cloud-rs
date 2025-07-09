#![allow(dead_code)]

use crate::error::ApiResult;
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
    pub fn data(self) -> T {
        self.data.unwrap()
    }
    pub fn new<M: AsRef<str>>(code: i32, msg: M, data: Option<T>) -> Self {
        ApiResponse {
            code,
            msg: String::from(msg.as_ref()),
            data,
        }
    }
    pub fn ok(data: Option<T>) -> Self {
        ApiResponse::new(0, "OK", data)
    }
    pub fn okk(data: Option<T>) -> ApiResult<T> {
        Ok(ApiResponse::ok(data))
    }
    pub fn err<M: AsRef<str>>(code: i32, msg: M) -> Self {
        ApiResponse::new(code, msg, None)
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
