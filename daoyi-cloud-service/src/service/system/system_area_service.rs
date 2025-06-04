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

pub async fn get_area_by_ip(
    login_user: OAuth2AccessTokenCheckRespDTO,
    csv_path: String,
    xdb_path: String,
    ip: String,
) -> AppResult<String> {
    let result = area_list(login_user, &csv_path, false).await?;
    let list: Vec<AreaRespVo> = result.list().to_vec();
    // 1. 加载xdb数据库文件
    let searcher = ip2region::Searcher::new(xdb_path)
        .map_err(|e| AppError::internal(format!("加载xdb数据库文件失败:{}", e.to_string())))?;
    let result = searcher
        .search(&ip)
        .map_err(|e| AppError::internal(format!("查询IP失败:{}", e.to_string())))?;
    let area_id: i64 = result
        .parse()
        .map_err(|_| AppError::internal(format!("区域编号转化失败:{}", result)))?;
    let area = list
        .iter()
        .find(|a| a.id == area_id)
        .ok_or_else(|| AppError::internal("未找到对应区域"))?;
    let mut res_name = area.name.to_owned();
    let mut parent = list.iter().find(|a| a.id == area.parent_id);
    while parent.is_some() {
        res_name = format!("{}-{}", &parent.unwrap().name, &res_name);
        parent = list.iter().find(|a| a.id == parent.unwrap().parent_id);
    }
    Ok(res_name.to_owned())
}
