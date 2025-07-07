use crate::service::system::user::admin_user_service::AdminUserService;
use daoyi_cloud_common::enums::EnumItemExt;
use daoyi_cloud_common::enums::login_log_type_enum::LoginLogTypeEnum;
use daoyi_cloud_common::enums::login_result_enum::LoginResultEnum;
use daoyi_cloud_common::enums::user_type_enum::UserTypeEnum;
use daoyi_cloud_common::error::biz_error::{
    AUTH_LOGIN_BAD_CREDENTIALS, AUTH_LOGIN_CAPTCHA_CODE_ERROR, AUTH_LOGIN_USER_DISABLED,
};
use daoyi_cloud_config::config;
use daoyi_cloud_config::config::jwt::{Principal, get_jwt};
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
        if params.social_type.is_some() {
            // todo!("TODO: AdminAuthService::bindSocialUser")
        }
        // 创建 Token 令牌，记录登录日志
        Self::create_token_after_login_success(
            db,
            &model.id,
            model.username.as_str(),
            &LoginLogTypeEnum::LoginUsername,
        )
        .await
    }

    pub async fn create_token_after_login_success(
        db: &DatabaseConnection,
        user_id: &i64,
        username: &str,
        log_type: &LoginLogTypeEnum,
    ) -> anyhow::Result<AuthLoginRespVo> {
        // 插入登陆日志
        Self::create_login_log(db, user_id, username, log_type, &LoginResultEnum::Success).await?;
        // 创建访问令牌
        let principal = Principal {
            id: user_id.to_owned(),
            user_type: UserTypeEnum::ADMIN.value(),
            info: "".to_string(),
            tenant_id: 0,
            scopes: vec![],
            expires_time: Default::default(),
            terminal_id: "".to_string(),
        };
        let access_token = get_jwt().encode(principal)?;
        // 构建返回结果
        todo!("TODO: AdminAuthService::create_token_after_login_success")
    }

    pub async fn create_login_log(
        db: &DatabaseConnection,
        user_id: &i64,
        username: &str,
        log_type: &LoginLogTypeEnum,
        login_result: &LoginResultEnum,
    ) -> anyhow::Result<()> {
        // 插入登录日志
        // 更新最后登录时间
        Ok(())
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
