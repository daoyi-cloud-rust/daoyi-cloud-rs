use crate::models::common_result::CommonResult;
use salvo::http::{ParseError, StatusCode, StatusError};
use salvo::oapi::{self, EndpointOutRegister, ToSchema};
use salvo::prelude::*;
use std::io;
use std::io::ErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("public: `{0}`")]
    Public(String),
    #[error("internal: `{0}`")]
    Internal(String),
    #[error("salvo internal error: `{0}`")]
    Salvo(#[from] ::salvo::Error),
    #[error("http status error: `{0}`")]
    HttpStatus(#[from] StatusError),
    #[error("http parse error:`{0}`")]
    HttpParse(#[from] ParseError),
    #[error("anyhow error:`{0}`")]
    Anyhow(#[from] anyhow::Error),
    #[error("seaorm db error:`{0}`")]
    Seaorm(#[from] sea_orm::DbErr),
    #[error("validation error:`{0}`")]
    Validation(#[from] validator::ValidationErrors),

    /// component not exists
    #[error("{0} component not exists")]
    ComponentNotExist(&'static str),

    /// `.env` file reading failed
    #[error(transparent)]
    EnvError(#[from] dotenvy::Error),

    /// File IO Error
    #[error(transparent)]
    IOError(#[from] io::Error),

    /// Configuration merge error in toml file
    #[error("merge toml error: {0}")]
    TomlMergeError(String),

    /// tokio asynchronous task join failed
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),

    /// toml file parsing error
    #[error(transparent)]
    TomlParseError(#[from] toml::de::Error),

    /// Deserialization of configuration in toml file to rust struct failed
    #[error("Failed to deserialize the configuration of prefix \"{0}\": {1}")]
    DeserializeErr(&'static str, toml::de::Error),
}
impl AppError {
    pub fn public<S: Into<String>>(msg: S) -> Self {
        Self::Public(msg.into())
    }

    pub fn internal<S: Into<String>>(msg: S) -> Self {
        Self::Internal(msg.into())
    }

    /// Failed to read file io
    pub fn from_io(kind: ErrorKind, msg: &str) -> Self {
        AppError::IOError(io::Error::new(kind, msg))
    }
}

/// Contains the return value of AppError
pub type Result<T> = std::result::Result<T, AppError>;

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let code = match &self {
            Self::HttpStatus(e) => e.code,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        res.status_code(code);
        let data = match self {
            Self::Salvo(e) => {
                tracing::error!(error = ?e, "salvo error");
                StatusError::internal_server_error().brief("Unknown error happened in salvo.")
            }
            Self::Public(msg) => StatusError::internal_server_error().brief(msg),
            Self::Internal(msg) => {
                tracing::error!(msg = msg, "internal error");
                StatusError::internal_server_error()
            }
            Self::HttpStatus(e) => e,
            e => StatusError::internal_server_error()
                .brief(format!("{e}"))
                .cause(e),
        };
        res.render(data);
    }
}
impl EndpointOutRegister for AppError {
    fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            oapi::Response::new("系统错误").add_content(
                "application/json",
                CommonResult::<()>::to_schema(components),
            ),
        );
        operation.responses.insert(
            StatusCode::NOT_FOUND.as_str(),
            oapi::Response::new("未找到请求").add_content(
                "application/json",
                CommonResult::<()>::to_schema(components),
            ),
        );
        operation.responses.insert(
            StatusCode::BAD_REQUEST.as_str(),
            oapi::Response::new("参数错误").add_content(
                "application/json",
                CommonResult::<()>::to_schema(components),
            ),
        );
    }
}
