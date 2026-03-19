use std::{collections::VecDeque, sync::Arc};

use reqwest::StatusCode;
use tokio::sync::Mutex;

use crate::{
    error::{AppServerResult, ServerErrorResponse},
    integrations::mailersend::client::MailersendClientTrait,
};

use super::email;

type RequestList<T> = Arc<Mutex<VecDeque<T>>>;
type ResponseList<T> = Arc<Mutex<VecDeque<AppServerResult<T>>>>;

#[derive(Clone, Debug, Default)]
pub struct MockClientRequests {
    pub send_email: RequestList<email::send_email::RequestBody>,
}

#[derive(Clone, Debug, Default)]
pub struct MockClientResponses {
    pub send_email: ResponseList<()>,
}

#[derive(Clone, Debug, Default)]
pub struct MockMailersendClient {
    pub requests: MockClientRequests,
    pub responses: MockClientResponses,
}

fn no_mock_response_error() -> ServerErrorResponse {
    ServerErrorResponse::new(
        StatusCode::IM_A_TEAPOT,
        1000,
        "No mock response set".to_string(),
    )
}

#[async_trait::async_trait]
impl MailersendClientTrait for MockMailersendClient {
    async fn send_email(&self, req: &email::send_email::RequestBody) -> AppServerResult {
        tracing::debug!(?req, "mock request");
        self.requests.send_email.lock().await.push_back(req.clone());
        match self.responses.send_email.lock().await.pop_front() {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }
}
