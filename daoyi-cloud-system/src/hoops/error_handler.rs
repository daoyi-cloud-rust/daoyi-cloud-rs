use crate::models::common_result::CommonResult;
use salvo::prelude::*;
use salvo::{FlowCtrl, Response, handler};

#[handler]
pub async fn http_error_handler(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
    res.render(CommonResult::<String>::build(
        res.status_code.unwrap(),
        None,
    ));
    res.status_code(StatusCode::OK);
    ctrl.skip_rest();
}
