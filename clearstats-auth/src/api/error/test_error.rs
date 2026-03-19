use serde::Serialize;
use shared_lib::error::{AppServerResult, ServerSuccessResponse};

#[derive(Clone, Debug, Serialize)]
pub struct ResponseBody {
    success: bool,
}

pub async fn handler() -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let resp = ResponseBody { success: true };
    test_function(&resp);
    Ok(ServerSuccessResponse::new(resp))
}

fn test_function(resp: &ResponseBody) {
    tracing::trace!(?resp, "Testing error");
    tracing::debug!(?resp, "Testing error");
    tracing::warn!(?resp, "Testing error");
    tracing::error!(?resp, "Testing error");
}
