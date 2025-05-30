use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use daoyi_cloud_models::models::system::permission_check_req_vo::PermissionCheckReqVO;
use daoyi_cloud_service::service::system::permission_service;
use salvo::Writer;
use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;

/// 判断是否有权限，任一一个即可
#[endpoint(tags("RPC 服务 - 权限"))]
pub async fn has_any_permission(params: JsonBody<PermissionCheckReqVO>) -> JsonResult<String> {
    let vo = params.into_inner();
    let res = permission_service::has_any_permissions(vo.user_id, vo.permissions).await;
    json_ok(res.to_string())
}
