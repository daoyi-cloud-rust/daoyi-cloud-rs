use crate::response::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("Method Not Allowed")]
    MethodNotAllowed,
    #[error("Db Error: {0}")]
    DbError(#[from] sea_orm::DbErr),
    #[error("{0}")]
    Biz(String),
    #[error("Error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz(_) => StatusCode::OK,
            ApiError::Internal(_) | ApiError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
