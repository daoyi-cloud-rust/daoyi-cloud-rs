use crate::AppError;
use salvo::http::StatusCode;
use salvo::prelude::*;
use salvo::{Response, Scribe};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommonResult<T> {
    code: u16,
    data: Option<T>,
    msg: String,
}

impl<T> CommonResult<T> {
    pub fn empty_success() -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: None,
            msg: StatusCode::OK.to_string(),
        }
    }
    pub fn success(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            msg: StatusCode::OK.to_string(),
        }
    }

    pub fn build(code: StatusCode, data: Option<T>) -> Self {
        Self {
            code: code.as_u16(),
            data,
            msg: code.to_string(),
        }
    }
    pub fn error(e: anyhow::Error) -> Self {
        let code = if e.downcast_ref::<AppError>().is_some() {
            match e.downcast_ref::<AppError>() {
                Some(AppError::HttpStatus(status)) => status.code.as_u16(),
                _ => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
        } else {
            StatusCode::INTERNAL_SERVER_ERROR.as_u16()
        };
        Self {
            code,
            data: None,
            msg: e.to_string(),
        }
    }
}

impl<T: Serialize + Send> Scribe for CommonResult<T> {
    fn render(self, res: &mut Response) {
        res.render(Json(self));
    }
}
