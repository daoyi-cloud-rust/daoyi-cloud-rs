use daoyi_cloud_hoops::hoops::{auth_middleware, tenant_middleware};
use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use salvo::Router;
use salvo::logging::Logger;
use salvo::oapi::{OpenApi, endpoint};
use salvo::prelude::Scalar;

pub fn root(routers: Router) -> Router {
    let router = Router::new()
        .hoop(Logger::new())
        .hoop(tenant_middleware::tenant_middleware)
        .hoop(auth_middleware::auth_middleware)
        .get(root_handler)
        .push(routers);
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar"))
}

#[endpoint(tags("根路由"))]
pub fn root_handler() -> JsonResult<String> {
    json_ok(String::from("Welcome to daoyi-cloud-rs."))
}
