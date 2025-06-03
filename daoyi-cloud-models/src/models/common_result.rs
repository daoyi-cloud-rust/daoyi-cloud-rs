use crate::models::error::AppError;
use salvo::{Response, Scribe, http::StatusCode, oapi, prelude::*};
use serde::{Deserialize, Serialize};
use std::any::type_name;

pub type AppResult<T> = Result<T, AppError>;
pub type JsonResult<T> = Result<CommonResult<T>, AppError>;
pub type EmptyResult = Result<CommonResult<Empty>, AppError>;

pub fn json_ok<T>(data: T) -> JsonResult<T> {
    Ok(CommonResult::success(data))
}
#[derive(Serialize, ToSchema, Clone, Copy, Debug)]
pub struct Empty {}

impl EndpointOutRegister for Empty {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation.responses.insert(
            StatusCode::OK.canonical_reason().unwrap(),
            Self::to_response(components),
        );
    }
}

impl ToResponse for Empty {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}

pub fn empty_ok() -> JsonResult<Empty> {
    Ok(CommonResult::success(Empty {}))
}

/// 通用返回
#[derive(Debug, Serialize, ToSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonResult<T> {
    /// 状态码
    code: u64,
    /// 数据
    data: Option<T>,
    /// 错误信息
    msg: String,
}

impl<T> CommonResult<T> {
    pub fn msg(self) -> String {
        self.msg
    }
    pub fn code(&self) -> u64 {
        self.code
    }
    pub fn data(self) -> Option<T> {
        self.data
    }

    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    pub fn is_fail(&self) -> bool {
        !self.is_success()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn empty_success() -> Self {
        Self {
            code: 0,
            data: None,
            msg: StatusCode::OK.canonical_reason().unwrap().to_string(),
        }
    }
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            data: Some(data),
            msg: StatusCode::OK.canonical_reason().unwrap().to_string(),
        }
    }

    pub fn from_status_code(code: StatusCode, data: Option<T>, msg: Option<String>) -> Self {
        Self {
            code: u64::from(code.as_u16()),
            data,
            msg: msg.unwrap_or_else(|| code.canonical_reason().unwrap().to_string()),
        }
    }

    pub fn error(e: anyhow::Error) -> Self {
        let res = if e.downcast_ref::<AppError>().is_some() {
            match e.downcast_ref::<AppError>() {
                Some(AppError::Biz(code, msg)) => Self {
                    code: code.to_owned(),
                    data: None,
                    msg: msg.to_owned(),
                },
                Some(AppError::HttpStatus(status)) => Self {
                    code: u64::from(status.code.as_u16()),
                    data: None,
                    msg: status.brief.to_string(),
                },
                _ => Self {
                    code: u64::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
                    data: None,
                    msg: e.to_string(),
                },
            }
        } else {
            Self {
                code: u64::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
                data: None,
                msg: e.to_string(),
            }
        };
        res
    }
}

impl<T: Serialize + Send> Scribe for CommonResult<T> {
    fn render(self, res: &mut Response) {
        res.render(Json(self));
    }
}

impl<T> EndpointOutRegister for CommonResult<T>
where
    T: ToSchema + EndpointOutRegister,
{
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation.responses.insert(
            StatusCode::OK.canonical_reason().unwrap(),
            Self::to_response(components),
        );
        T::register(components, operation);
    }
}

impl<C> ToResponse for CommonResult<C>
where
    C: ToSchema,
{
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = <C as ToSchema>::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}

pub fn to_common_response(
    components: &mut oapi::Components,
    type_name: &str,
    schema_ref: oapi::RefOr<oapi::schema::Schema>,
) -> oapi::RefOr<oapi::response::Response> {
    let response = oapi::Response::new("成功").add_content(
        "application/json",
        oapi::Content::new(
            oapi::Object::new()
                .property(
                    "code",
                    oapi::Object::new()
                        .description("状态码")
                        .schema_type(oapi::schema::SchemaType::basic(
                            oapi::schema::BasicType::Integer,
                        ))
                        .format(oapi::SchemaFormat::KnownFormat(oapi::KnownFormat::Int32))
                        .example(0),
                )
                .required("code")
                .property(
                    "msg",
                    oapi::Object::new()
                        .description("错误信息")
                        .schema_type(oapi::schema::SchemaType::basic(
                            oapi::schema::BasicType::String,
                        ))
                        .format(oapi::SchemaFormat::KnownFormat(oapi::KnownFormat::String))
                        .example("success"),
                )
                .required("msg")
                .property("data", schema_ref),
        ),
    );
    components.responses.insert(type_name, response);
    oapi::RefOr::Ref(oapi::Ref::new(format!(
        "#/components/responses/{type_name}"
    )))
}
