use crate::rpc_service::system::permission_service::has_any_permission;
use crate::rpc_service::system::system_oauth2_access_token::check_access_token_redis;
use daoyi_cloud_config::config;
use daoyi_cloud_models::models::common_result::CommonResult;
use daoyi_cloud_models::models::system::permission_check_req_vo::PermissionCheckReqVO;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use daoyi_cloud_utils::utils::path_matches;
use salvo::http::StatusCode;
use salvo::http::header::ToStrError;
use salvo::{Depot, FlowCtrl, Handler, Request, Response, async_trait, handler};

pub struct SS {
    permissions: Vec<String>,
}

impl SS {
    pub fn has_permission(permission: String) -> Self {
        Self {
            permissions: vec![permission],
        }
    }
}

#[async_trait]
impl Handler for SS {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let auth_middleware_config = &config::get().auth;
        let login_user_key = auth_middleware_config.login_user_key.as_str();
        if let Ok(login_user) = depot.get::<OAuth2AccessTokenCheckRespDTO>(login_user_key) {
            if has_any_permission(PermissionCheckReqVO {
                user_id: login_user.user_id,
                permissions: self.permissions.to_owned(),
            })
            .await
            {
                ctrl.call_next(req, depot, res).await;
                return;
            } else {
                res.render(CommonResult::<String>::build(
                    StatusCode::FORBIDDEN,
                    None,
                    Some("当前操作没有权限.".to_string()),
                ));
                ctrl.skip_rest();
                return;
            }
        }
        ctrl.call_next(req, depot, res).await;
    }
}

#[handler]
pub async fn auth_middleware(
    &self,
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let auth_middleware_config = &config::get().auth;
    let header_name = auth_middleware_config.header_name.as_str();
    let login_user_key = auth_middleware_config.login_user_key.as_str();
    let tenant_middleware_config = &config::get().tenant;
    if let Some(header_value) = req.headers().get(header_name) {
        let authorization: Result<&str, ToStrError> = header_value.to_str();
        if let Ok(authorization) = authorization {
            if let Some(token) = authorization.strip_prefix(auth_middleware_config.prefix.as_str())
            {
                if let Ok(tenant_id) =
                    depot.get::<i64>(tenant_middleware_config.header_name.as_str())
                {
                    println!("tenant_id: {}", tenant_id);
                    let result = check_access_token_redis(token).await;
                    match result {
                        Ok(resp) => {
                            if let Some(resp_dto) = resp.data() {
                                depot.insert(login_user_key, resp_dto);
                            } else {
                                res.render(CommonResult::<String>::build(
                                    StatusCode::UNAUTHORIZED,
                                    None,
                                    Some("Token无效.".to_string()),
                                ));
                                ctrl.skip_rest();
                                return;
                            }
                        }
                        Err(err) => {
                            res.render(CommonResult::<String>::error(anyhow::Error::from(err)));
                            ctrl.skip_rest();
                            return;
                        }
                    }
                } else {
                    res.render(CommonResult::<String>::build(
                        StatusCode::UNAUTHORIZED,
                        None,
                        Some("租户ID错误.".to_string()),
                    ));
                    ctrl.skip_rest();
                    return;
                }
            } else {
                res.render(CommonResult::<String>::build(
                    StatusCode::UNAUTHORIZED,
                    None,
                    Some("Token无效.".to_string()),
                ));
                ctrl.skip_rest();
                return;
            }
        } else {
            res.render(CommonResult::<String>::build(
                StatusCode::UNAUTHORIZED,
                None,
                Some("Token无效.".to_string()),
            ));
            ctrl.skip_rest();
            return;
        }
    } else {
        let path = String::from(req.uri().path());
        if !path_matches::path_any_matches(&auth_middleware_config.ignore_urls, &path) {
            res.render(CommonResult::<String>::build(
                StatusCode::UNAUTHORIZED,
                None,
                Some("未登录.".to_string()),
            ));
            ctrl.skip_rest();
            return;
        }
    }
    ctrl.call_next(req, depot, res).await;
}
