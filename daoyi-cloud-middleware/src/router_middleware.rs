use crate::auth_middleware::auth_middleware;
use crate::tenant_middleware::tenant_middleware;
use spring_web::Router;
use spring_web::axum::middleware;

pub fn root_router() -> Router {
    Router::new()
        .merge(spring_web::handler::auto_router())
        // .layer(TimeoutLayer::new(Duration::from_secs(120)))
        .layer(middleware::from_fn(tenant_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
