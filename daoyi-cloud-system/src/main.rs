use spring::App;
use spring_sea_orm::{DbConn, SeaOrmPlugin};
use spring_web::axum::response::{IntoResponse, Response};
use spring_web::{get, Router, WebConfigurator, WebPlugin};
use std::path::PathBuf;
use std::time::Duration;
use spring_web::axum::{body, middleware};
use spring_web::axum::middleware::Next;
use spring_web::extractor::{Component, Request};
use spring_web::middleware::timeout::TimeoutLayer;

#[tokio::main]
async fn main() {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/app.toml");
    App::new()
        .use_config_file(config_path.to_str().unwrap())
        .add_plugin(SeaOrmPlugin)
        .add_plugin(WebPlugin)
        .add_router(router())
        .run()
        .await
}

fn router() -> Router {
    Router::new()
        .merge(spring_web::handler::auto_router())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        // .layer(middleware::from_fn(problem_middleware))
}

// async fn problem_middleware(
//     Component(db): Component<DbConn>,
//     request: Request,
//     next: Next,
// ) -> Response {
//     let uri = request.uri().path().to_string();
//     let response = next.run(request).await;
//     let status = response.status();
//     if status.is_client_error() || status.is_server_error() {
//         let bytes = body::to_bytes(response.into_body(), usize::MAX)
//             .await
//             .expect("server body read failed");
//         let msg = String::from_utf8(bytes.to_vec()).expect("read body to string failed");
// 
//         // error log into db
//         let _ = sqlx::query("insert into error_log (msg, created_at) values ($1, now())")
//             .bind(&msg)
//             .execute(&db)
//             .await;
// 
//         problemdetails::new(status)
//             .with_instance(uri)
//             .with_title(status.canonical_reason().unwrap_or("error"))
//             .with_detail(msg)
//             .into_response()
//     } else {
//         response
//     }
// }

#[get("/")]
async fn hello_word() -> impl IntoResponse {
    "hello word"
}
