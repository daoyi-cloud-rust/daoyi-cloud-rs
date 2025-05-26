use salvo::{http::StatusCode, oapi, prelude::*, Response, Scribe};
use serde::Serialize;
use crate::models::error::AppError;

pub type AppResult<T> = Result<T, AppError>;
pub type JsonResult<T> = Result<CommonResult<T>, AppError>;
pub type EmptyResult = Result<CommonResult<Empty>, AppError>;

pub fn json_ok<T>(data: T) -> JsonResult<T> {
    Ok(CommonResult::success(data))
}
#[derive(Serialize, ToSchema, Clone, Copy, Debug)]
pub struct Empty {}
pub fn empty_ok() -> JsonResult<Empty> {
    Ok(CommonResult::success(Empty {}))
}

/// 通用返回
#[derive(Debug, Serialize, ToSchema)]
pub struct CommonResult<T> {
    /// 状态码
    code: u16,
    /// 数据
    data: Option<T>,
    /// 错误信息
    msg: String,
}

impl<T> CommonResult<T> {
    pub fn empty_success() -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: None,
            msg: StatusCode::OK.to_string(),
        }
    }
    pub fn success(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            msg: StatusCode::OK.to_string(),
        }
    }

    pub fn build(code: StatusCode, data: Option<T>, msg: Option<String>) -> Self {
        Self {
            code: code.as_u16(),
            data,
            msg: msg.unwrap_or_else(|| code.to_string()),
        }
    }
    pub fn error(e: anyhow::Error) -> Self {
        let code = if e.downcast_ref::<AppError>().is_some() {
            match e.downcast_ref::<AppError>() {
                Some(AppError::HttpStatus(status)) => status.code.as_u16(),
                _ => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
        } else {
            StatusCode::INTERNAL_SERVER_ERROR.as_u16()
        };
        Self {
            code,
            data: None,
            msg: e.to_string(),
        }
    }
}

impl<T: Serialize + Send> Scribe for CommonResult<T> {
    fn render(self, res: &mut Response) {
        res.render(Json(self));
    }
}

impl<T> EndpointOutRegister for CommonResult<T>
where
    T: ToSchema,
{
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert("200", Self::to_response(components));
    }
}

impl<C> ToResponse for CommonResult<C>
where
    C: ToSchema,
{
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = <C as ToSchema>::to_schema(components);
        let response = oapi::Response::new("成功")
            .add_content(
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
        components.responses.insert("CommonResult", response);
        oapi::RefOr::Ref(oapi::Ref::new(format!(
            "#/components/responses/{}",
            "CommonResult"
        )))
    }
}
