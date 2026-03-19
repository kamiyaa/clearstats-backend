pub mod email;
pub mod mock_client;

mod mailersend_client;
pub use mailersend_client::*;

use crate::error::AppServerResult;

#[async_trait::async_trait]
pub trait MailersendClientTrait {
    async fn send_email(&self, req: &email::send_email::RequestBody) -> AppServerResult;
}
