use axum::debug_handler;
use axum::response::IntoResponse;

#[debug_handler]
pub async fn create_dept() -> impl IntoResponse {
    "create_dept"
}
