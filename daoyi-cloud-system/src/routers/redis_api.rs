use daoyi_cloud_config::redis_util;
use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use daoyi_cloud_models::models::system::clear_redis_cache_req_vo::ClearRedisCacheReqVO;
use salvo::Writer;
use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;

/// 清理Redis缓存
#[endpoint(tags("管理后台 - Redis服务"))]
pub async fn clear_cache(params: JsonBody<ClearRedisCacheReqVO>) -> JsonResult<String> {
    let vo = params.into_inner();
    let res = json_ok("true".to_string());
    if let Some(keys) = vo.keys {
        if !keys.is_empty() {
            for key in keys {
                redis_util::clear_cached_key(&key).await;
            }
            return res;
        }
    }
    redis_util::clear_all_cached_keys().await;
    res
}
