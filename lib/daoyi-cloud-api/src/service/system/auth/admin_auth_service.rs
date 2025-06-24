use crate::service::system::user::admin_user_service::AdminUserService;
use daoyi_cloud_common::enums::login_log_type_enum::LoginLogTypeEnum;
use daoyi_cloud_common::error::biz_error::{
    AUTH_LOGIN_BAD_CREDENTIALS, AUTH_LOGIN_CAPTCHA_CODE_ERROR, AUTH_LOGIN_USER_DISABLED,
};
use daoyi_cloud_config::config;
use daoyi_cloud_entity::entity::system::system_users;
use daoyi_cloud_entity::vo::auth::{AuthLoginReqVo, AuthLoginRespVo, CaptchaVerificationReqVO};
use sea_orm::DatabaseConnection;

pub struct AdminAuthService;

impl AdminAuthService {
    pub async fn login(
        db: &DatabaseConnection,
        params: AuthLoginReqVo,
    ) -> anyhow::Result<AuthLoginRespVo> {
        // 校验验证码
        Self::validate_captcha(&params).await?;
        // 使用账号密码，进行登录
        let model = Self::authenticate(db, &params.username, &params.password).await?;
        // 如果 socialType 非空，说明需要绑定社交用户
        // 创建 Token 令牌，记录登录日志
        todo!("TODO: AdminAuthService::login");
    }

    pub async fn authenticate(
        db: &DatabaseConnection,
        username: &String,
        password: &String,
    ) -> anyhow::Result<system_users::Model> {
        let _log_type_enum = LoginLogTypeEnum::LoginUsername;
        // 校验账号是否存在
        let option = AdminUserService::select_by_username(db, username).await?;
        if option.is_none() {
            return Err(anyhow::Error::from(
                AUTH_LOGIN_BAD_CREDENTIALS.to_app_error(),
            ));
        }
        let model = option.unwrap();
        let checked =
            AdminUserService::is_password_match(password, model.password.as_ref().unwrap()).await?;
        if !checked {
            return Err(anyhow::Error::from(
                AUTH_LOGIN_BAD_CREDENTIALS.to_app_error(),
            ));
        }
        // 校验是否禁用
        if model.is_disabled() {
            return Err(anyhow::Error::from(AUTH_LOGIN_USER_DISABLED.to_app_error()));
        }
        Ok(model)
    }

    async fn validate_captcha<T: CaptchaVerificationReqVO>(_params: &T) -> anyhow::Result<()> {
        let enable = &config::get().captcha().enable;
        if !enable {
            return Ok(());
        }
        Err(anyhow::Error::from(
            AUTH_LOGIN_CAPTCHA_CODE_ERROR.to_app_error_args(vec!["没有实现验证码校验逻辑哦~"]),
        ))
    }
}
