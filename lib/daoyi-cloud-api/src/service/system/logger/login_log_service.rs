use sea_orm::DatabaseConnection;

pub struct LoginLogService;

impl LoginLogService {
    pub async fn create_login_log(db: &DatabaseConnection) -> anyhow::Result<()> {
        Ok(())
    }
}
