use crate::{
    error::AppServerResult,
    integrations::mailersend::client::{MailersendClientTrait, email},
};

#[derive(Clone, Debug)]
pub struct MailersendClient {
    pub api_key: String,
}

impl MailersendClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait::async_trait]
impl MailersendClientTrait for MailersendClient {
    async fn send_email(&self, req: &email::send_email::RequestBody) -> AppServerResult {
        email::send_email::handler(&self.api_key, req).await
    }
}
