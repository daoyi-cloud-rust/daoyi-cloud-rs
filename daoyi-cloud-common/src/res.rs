use crate::error::CusErr;
use serde::{Deserialize, Serialize};
use serde_json::json;
use spring_web::axum::{
    Json,
    extract::rejection::{JsonRejection, QueryRejection},
    response::{IntoResponse, Response},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Res<T> {
    code: i32,
    data: Option<T>,
    message: String,
}

impl<T> Res<T> {
    pub fn empty_success() -> Self {
        Self {
            code: 200,
            data: None,
            message: "success".to_string(),
        }
    }
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            message: "success".to_string(),
        }
    }
    pub fn error(e: anyhow::Error) -> Self {
        let code = if e.downcast_ref::<CusErr>().is_some() {
            match e.downcast_ref::<CusErr>() {
                Some(CusErr::AppRuleError(_)) => 400,
                _ => 404,
            }
        } else {
            500
        };
        Self {
            code,
            data: None,
            message: e.to_string(),
        }
    }
}

impl<T: Serialize> IntoResponse for Res<T> {
    fn into_response(self) -> Response {
        let val = json!(self);
        Json(val).into_response()
    }
}

impl From<JsonRejection> for Res<()> {
    fn from(value: JsonRejection) -> Self {
        Self {
            code: value.status().as_u16().into(),
            data: None,
            message: value.body_text(),
        }
    }
}

impl From<QueryRejection> for Res<()> {
    fn from(value: QueryRejection) -> Self {
        Self {
            code: value.status().as_u16().into(),
            data: None,
            message: value.body_text(),
        }
    }
}
