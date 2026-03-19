use serde::Serialize;

use crate::error::{AppServerResult, ServerSuccessResponse};

#[derive(Serialize)]
pub struct ResponseBody {
    success: bool,
}

pub async fn health_check() -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let resp = ResponseBody { success: true };
    Ok(ServerSuccessResponse::new(resp))
}
