use daoyi_cloud_models::models::common_result::CommonResult;
use salvo::http::ResBody;
use salvo::prelude::*;
use salvo::{handler, FlowCtrl, Response};

#[handler]
pub async fn http_error_handler(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
    let error_msg = match res.take_body() {
        ResBody::Error(e) => e.brief,
        _ => "Unknown error".to_string(),
    };

    res.render(CommonResult::<String>::build(
        res.status_code.unwrap(),
        None,
        Some(error_msg),
    ));
    res.status_code(StatusCode::OK);
    ctrl.skip_rest();
}
