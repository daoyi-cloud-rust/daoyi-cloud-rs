use daoyi_cloud_models::models::common_result::CommonResult;
use salvo::http::ResBody;
use salvo::prelude::*;
use salvo::{FlowCtrl, Response, handler};

#[handler]
pub async fn http_error_handler(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
    let error_msg = match res.take_body() {
        ResBody::Error(e) => e.brief,
        _ => res
            .status_code
            .unwrap()
            .canonical_reason()
            .unwrap()
            .to_string(),
    };

    res.render(CommonResult::<String>::from_status_code(
        res.status_code.unwrap(),
        None,
        Some(error_msg),
    ));
    res.status_code(StatusCode::OK);
    ctrl.skip_rest();
}
