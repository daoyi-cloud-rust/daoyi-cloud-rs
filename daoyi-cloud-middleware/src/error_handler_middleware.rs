use serde_json::json;
use spring_web::axum;
use spring_web::axum::response::{IntoResponse, Response};
use spring_web::axum::BoxError;
use spring_web::axum::http::StatusCode;

pub async fn error_handler_middleware(
    error: BoxError
) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    // let response = next.run(request).await;
    // 
    // if response.status().is_server_error() {
    //     // 获取错误详情（如果有）
    //     let error_text = response
    //         .extensions()
    //         .get::<BoxError>()
    //         .map(|e| e.to_string())
    //         .unwrap_or_else(|| "Unknown error".to_string());
    // 
    //     // 返回结构化的错误响应
    //     let error_response = json!({
    //         "error": true,
    //         "message": error_text,
    //         "status": response.status().as_u16(),
    //     });
    // 
    //     return (response.status(), axum::Json(error_response)).into_response();
    // }
    // 
    // response
}
