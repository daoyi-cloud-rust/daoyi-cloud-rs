pub mod common_result;
pub mod error;
pub mod system;

use salvo::oapi::ToSchema;
use serde::Serialize;

#[derive(Serialize, ToSchema, Debug)]
pub struct SafeUser {
    pub id: String,
    pub username: String,
}
