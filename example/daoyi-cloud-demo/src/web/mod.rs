use axum::debug_handler;
use axum::http::{Method, header};
use axum::response::{IntoResponse, Response};
use daoyi_cloud_common::error::ApiError;
use daoyi_cloud_common::models::api_extract::path::Path;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "resources/static"]
#[include = "index.html"]
struct IndexHtml;

#[derive(Embed)]
#[folder = "resources/static"]
#[exclude = "index.html"]
struct StaticAssets;

struct StaticFile<T>(T);

impl<T: AsRef<str>> IntoResponse for StaticFile<T> {
    fn into_response(self) -> Response {
        let path = self.0.as_ref();
        match StaticAssets::get(path) {
            Some(file) => {
                let mime = file.metadata.mimetype();
                let body = file.data;
                ([(header::CONTENT_TYPE, mime)], body).into_response()
            }
            None => ApiError::NotFound.into_response(),
        }
    }
}

#[debug_handler]
pub async fn static_assets_handler(Path(path): Path<String>) -> impl IntoResponse {
    StaticFile(path).into_response()
}

#[debug_handler]
pub async fn index_handler(method: Method) -> impl IntoResponse {
    if method == Method::GET {
        let file = IndexHtml::get("index.html").expect("index.html不存在");
        ([(header::CONTENT_TYPE, "text/html")], file.data).into_response()
    } else {
        ApiError::NotFound.into_response()
    }
}
