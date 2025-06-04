use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::area_resp_vo::AreaRespVo;
use daoyi_cloud_service::service::get_current_user;
use daoyi_cloud_service::service::system::system_area_service;
use salvo::oapi::endpoint;
use salvo::oapi::extract::QueryParam;
use salvo::{Depot, Writer};

/// 获取地区树
#[endpoint(tags("管理后台 - 系统管理 - 地区"))]
pub async fn get_area_tree(
    refresh: QueryParam<bool, false>,
    depot: &mut Depot,
) -> JsonResult<PageResult<AreaRespVo>> {
    let csv_path = format!("{}/resources/area.csv", env!("CARGO_MANIFEST_DIR"));
    let login_user = get_current_user(depot);
    let list = system_area_service::area_list_tree(
        login_user,
        csv_path,
        refresh.into_inner().unwrap_or(false),
    )
    .await?;
    json_ok(list)
}
