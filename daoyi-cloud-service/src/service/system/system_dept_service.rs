use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::system::system_dept;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::system::dept_save_req_vo::DeptSaveReqVo;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use sea_orm::{ActiveModelTrait, Set};

pub async fn create_dept(
    login_user: OAuth2AccessTokenCheckRespDTO,
    req_vo: DeptSaveReqVo,
) -> AppResult<system_dept::Model> {
    let model = system_dept::ActiveModel {
        email: Set(req_vo.email),
        leader_user_id: Set(req_vo.leader_user_id),
        name: Set(req_vo.name),
        parent_id: Set(req_vo.parent_id.unwrap_or(0)),
        phone: Set(req_vo.phone),
        sort: Set(req_vo.sort),
        status: Set(req_vo.status),
        creator: Set(Some(login_user.user_id.to_string())),
        updater: Set(Some(login_user.user_id.to_string())),
        tenant_id: Set(login_user.tenant_id),
        ..Default::default()
    };
    let model = model.insert(db::pool()).await?;
    Ok(model)
}
