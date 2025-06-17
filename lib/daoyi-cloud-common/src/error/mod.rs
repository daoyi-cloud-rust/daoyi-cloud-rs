pub mod biz_error;

use crate::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_valid::ValidRejection;

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("服务器迷路了.")]
    NotFound,
    #[error("请求方法不支持.")]
    MethodNotAllowed,
    #[error("数据库错误: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("查询参数错误: {0}")]
    Query(#[from] QueryRejection),
    #[error("路径参数错误: {0}")]
    Path(#[from] PathRejection),
    #[error("Body参数错误: {0}")]
    Json(#[from] JsonRejection),
    #[error("参数校验失败: {0}")]
    Validation(String),
    #[error("{1}")]
    Biz(u64, String),
    #[error("错误: {0}")]
    Internal(#[from] anyhow::Error),
}

impl From<ValidRejection<ApiError>> for ApiError {
    fn from(value: ValidRejection<ApiError>) -> Self {
        match value {
            ValidRejection::Valid(err) => ApiError::Validation(err.to_string()),
            ValidRejection::Inner(err) => err,
        }
    }
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz(_, _) => StatusCode::OK,
            ApiError::Internal(_) | ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Query(_)
            | ApiError::Path(_)
            | ApiError::Json(_)
            | ApiError::Validation(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let body = axum::Json(ApiResponse::<()>::err_msg(self.to_string()));
        (status_code, body).into_response()
    }
}
