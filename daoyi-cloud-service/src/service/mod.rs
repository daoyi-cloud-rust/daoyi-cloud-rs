use daoyi_cloud_config::config;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use salvo::Depot;

pub mod system;

pub fn get_current_user(depot: &Depot) -> OAuth2AccessTokenCheckRespDTO {
    let auth_config = &config::get().auth;
    if let Ok(user) =
        depot.get::<OAuth2AccessTokenCheckRespDTO>(auth_config.login_user_key.as_str())
    {
        return user.to_owned();
    };
    OAuth2AccessTokenCheckRespDTO::default()
}
