use chrono::Local;
use daoyi_cloud_common::enums::EnumItemExt;
use daoyi_cloud_common::enums::common_status_enum::CommonStatusEnum;
use daoyi_cloud_common::error::biz_error::{TENANT_DISABLE, TENANT_EXPIRE, TENANT_NOT_EXISTS};
use daoyi_cloud_entity::entity::system::prelude::SystemTenant;
use daoyi_cloud_entity::entity::system::system_tenant;
use sea_orm::prelude::*;

pub struct TenantService;
impl TenantService {
    pub async fn valid_tenant(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
        let tenant = Self::get_tenant(db, &id).await?;
        let tenant = tenant.ok_or_else(|| anyhow::Error::from(TENANT_NOT_EXISTS.to_app_error()))?;
        if tenant.status == CommonStatusEnum::Disable.value() {
            return Err(anyhow::Error::from(
                TENANT_DISABLE.to_app_error_args(vec![&tenant.name]),
            ));
        }
        if tenant.expire_time.lt(&Local::now().naive_local()) {
            return Err(anyhow::Error::from(
                TENANT_EXPIRE.to_app_error_args(vec![&tenant.name]),
            ));
        }
        Ok(())
    }

    pub async fn get_tenant(
        db: &DatabaseConnection,
        id: &i64,
    ) -> anyhow::Result<Option<system_tenant::Model>> {
        let model = SystemTenant::find_by_id(id.to_owned()).one(db).await?;
        Ok(model)
    }
}
