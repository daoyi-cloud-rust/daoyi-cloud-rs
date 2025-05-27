use daoyi_cloud_config::config;
use daoyi_cloud_models::models::common_result::CommonResult;
use daoyi_cloud_utils::utils::path_matches;
use salvo::http::StatusCode;
use salvo::http::header::ToStrError;
use salvo::{Depot, FlowCtrl, Request, Response, handler};

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
    if let Some(header_value) = req.headers().get(header_name) {
        let authorization: Result<&str, ToStrError> = header_value.to_str();
        if let Ok(authorization) = authorization {
            if authorization.starts_with(auth_middleware_config.prefix.as_str()) {
                // 修改租户ID
                depot.insert(header_name, 1i64);
            } else {
                res.render(CommonResult::<String>::build(
                    StatusCode::UNAUTHORIZED,
                    None,
                    Some("Token无效.".to_string()),
                ));
                res.status_code(StatusCode::OK);
                ctrl.skip_rest();
                return;
            }
        } else {
            res.render(CommonResult::<String>::build(
                StatusCode::UNAUTHORIZED,
                None,
                Some("Token无效.".to_string()),
            ));
            res.status_code(StatusCode::OK);
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
            res.status_code(StatusCode::OK);
            ctrl.skip_rest();
            return;
        }
    }
    ctrl.call_next(req, depot, res).await;
}
