use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::error::AppError;
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::area_resp_vo::{AreaCsvVo, AreaRespVo};
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use daoyi_cloud_models::models::tree_utils;
use std::fs::File;

pub async fn area_list(
    _login_user: OAuth2AccessTokenCheckRespDTO,
    csv_path: &str,
    _refresh: bool,
) -> AppResult<PageResult<AreaRespVo>> {
    // 打开CSV文件
    let file = File::open(csv_path)?;
    // 创建CSV读取器
    let mut rdr = csv::Reader::from_reader(file);
    // 用于存储解析后的结构体列表
    let result: Vec<AreaCsvVo> = rdr
        .deserialize()
        .collect::<Result<_, _>>()
        .map_err(|e| AppError::internal(format!("CSV结构化失败:{}", e.to_string())))?;
    let mut result: Vec<AreaRespVo> = result
        .into_iter()
        .map(|vo| AreaRespVo::from(vo))
        .collect::<Vec<_>>();
    let total = result.len() as u64;
    result.sort_by_key(|a| a.id);

    Ok(PageResult::build(result, total, 1, total as i32))
}

pub async fn area_list_tree(
    login_user: OAuth2AccessTokenCheckRespDTO,
    csv_path: String,
    refresh: bool,
) -> AppResult<PageResult<AreaRespVo>> {
    let mut result = area_list(login_user, &csv_path, refresh).await?;
    let list: Vec<AreaRespVo> = result.list().to_vec();
    result.set_list(tree_utils::TreeUtil::<AreaRespVo>::build(list).build_tree());
    Ok(result)
}
