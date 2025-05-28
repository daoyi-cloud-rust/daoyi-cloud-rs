use daoyi_cloud_config::config;
use daoyi_cloud_models::models::common_result::CommonResult;
use daoyi_cloud_utils::utils::path_matches;
use salvo::http::StatusCode;
use salvo::{Depot, FlowCtrl, Request, Response, handler};

#[handler]
pub async fn tenant_middleware(
    &self,
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let tenant_middleware_config = &config::get().tenant;
    let header_name = tenant_middleware_config.header_name.as_str();
    if let Some(header_value) = req.headers().get(header_name) {
        let tenant_id: Result<i64, _> = header_value.to_str().unwrap_or_default().parse();
        if let Ok(tenant_id) = tenant_id {
            // 修改租户ID
            depot.insert(header_name, tenant_id);
        } else {
            res.render(CommonResult::<String>::build(
                StatusCode::BAD_REQUEST,
                None,
                Some("租户ID错误.".to_string()),
            ));
            res.status_code(StatusCode::OK);
            ctrl.skip_rest();
            return;
        }
    } else {
        let path = String::from(req.uri().path());
        if tenant_middleware_config.enabled()
            && !path_matches::path_any_matches(&tenant_middleware_config.ignore_urls, &path)
        {
            res.render(CommonResult::<String>::build(
                StatusCode::BAD_REQUEST,
                None,
                Some("请求的租户标识未传递，请进行排查.".to_string()),
            ));
            res.status_code(StatusCode::OK);
            ctrl.skip_rest();
            return;
        }
    }
    ctrl.call_next(req, depot, res).await;
}
