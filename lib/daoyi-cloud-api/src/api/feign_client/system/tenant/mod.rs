use daoyi_cloud_common::error::{ApiError, ApiResult};
use daoyi_cloud_common::response::ApiResponse;
use daoyi_cloud_config::config;
use daoyi_cloud_logger::logger;

pub struct TenantApi;
impl TenantApi {
    pub async fn valid_tenant(id: i64) -> ApiResult<bool> {
        let url = &config::get().tenant().valid_url;
        let url = format!("{url}?id={id}");
        let response = reqwest::get(url).await.map_err(|e| {
            logger::error!("RPC请求失败: {}", e);
            anyhow::Error::from(e)
        })?;
        if !response.status().is_success() {
            logger::info!("RPC请求失败: {}", response.status());
            return ApiResponse::okk(Some(false));
        }
        let json_str = response.text().await.map_err(|e| {
            logger::error!("RPC请求数据解析失败: {}", e);
            anyhow::Error::from(e)
        })?;
        let resp: ApiResponse<bool> = serde_json::from_str(&json_str).map_err(|e| {
            logger::error!("数据反序列化失败: {}", e);
            anyhow::Error::from(e)
        })?;
        ApiResponse::okk(Some(resp.data()))
    }
}
