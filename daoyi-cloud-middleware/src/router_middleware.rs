use crate::auth_middleware::auth_middleware;
use spring_web::axum::middleware;
use spring_web::Router;

pub fn root_router() -> Router {
    Router::new()
        .merge(spring_web::handler::auto_router())
        // .layer(TimeoutLayer::new(Duration::from_secs(120)))
        .layer(middleware::from_fn(auth_middleware))
}
