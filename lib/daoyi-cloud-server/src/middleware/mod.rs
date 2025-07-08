use axum::body::Body;
use axum::http::{Request, Response};
use daoyi_cloud_common::error::ApiError;
use daoyi_cloud_common::utils::path_matches::path_any_matches;
use daoyi_cloud_config::config;
use daoyi_cloud_config::config::jwt::{JWT, get_jwt};
use std::pin::Pin;
use std::sync::LazyLock;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

static AUTH_LAYER: LazyLock<AsyncRequireAuthorizationLayer<JWTAuth>> =
    LazyLock::new(|| AsyncRequireAuthorizationLayer::new(JWTAuth::new(get_jwt())));

#[derive(Clone)]
pub struct JWTAuth {
    jwt: &'static JWT,
}

impl JWTAuth {
    pub fn new(jwt: &'static JWT) -> Self {
        Self { jwt }
    }
}

impl AsyncAuthorizeRequest<Body> for JWTAuth {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send
                + 'static,
        >,
    >;

    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        let jwt = self.jwt;
        Box::pin(async move {
            let path = request.uri().path();
            let ignore_urls = &config::get().auth().ignore_urls;
            let header_key = &config::get().auth().header;
            let header_prefix = &config::get().auth().prefix;
            let token_in_header = request.headers().get(header_key);
            if path_any_matches(ignore_urls, path) {
                if token_in_header.is_none() {
                    return Ok(request);
                }
            }
            let token = token_in_header
                .map(|value| -> Result<_, ApiError> {
                    let token = value
                        .to_str()
                        .map_err(|_| {
                            ApiError::Unauthenticated(format!(
                                "{}请求头不是一个有效的字符串",
                                header_key
                            ))
                        })?
                        .strip_prefix(header_prefix)
                        .ok_or_else(|| {
                            ApiError::Unauthenticated(format!(
                                "{}请求头必须以{}开头",
                                header_key, header_prefix
                            ))
                        })?;
                    Ok(token)
                })
                .transpose()?
                .ok_or_else(|| {
                    ApiError::Unauthenticated(format!("{}请求头不能为空", header_key))
                })?;
            let principal = jwt.decode(token).map_err(|err| ApiError::Internal(err))?;
            request.extensions_mut().insert(principal);
            Ok(request)
        })
    }
}

pub fn get_auth_layer() -> &'static AsyncRequireAuthorizationLayer<JWTAuth> {
    &AUTH_LAYER
}
