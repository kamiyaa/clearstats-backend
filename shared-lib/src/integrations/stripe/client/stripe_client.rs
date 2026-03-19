use crate::error::AppServerResult;
use crate::integrations::stripe::client::customer::fetch_customer;
use crate::integrations::stripe::client::subscription::fetch_subscription;
use crate::integrations::stripe::client::usage_meter::send_usage_meter_event;
use crate::integrations::stripe::types::{CustomerObject, ListObject, SubscriptionObject};

use super::StripeClientTrait;
use super::checkout::{create_checkout, fetch_checkout_line_items};
use super::customer::create_customer;

#[derive(Clone, Debug)]
pub struct StripeClient {
    pub secret_key: String,
}

impl StripeClient {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }
}

#[async_trait::async_trait]
impl StripeClientTrait for StripeClient {
    async fn create_customer(
        &self,
        req: &create_customer::RequestBody,
    ) -> AppServerResult<create_customer::ResponseBody> {
        create_customer::handler(&self.secret_key, req).await
    }

    async fn create_checkout(
        &self,
        req: &create_checkout::RequestBody,
    ) -> AppServerResult<create_checkout::ResponseBody> {
        create_checkout::handler(&self.secret_key, req).await
    }

    async fn fetch_checkout_line_items(&self, checkout_id: &str) -> AppServerResult<ListObject> {
        fetch_checkout_line_items::handler(&self.secret_key, checkout_id).await
    }

    async fn fetch_customer(&self, customer_id: &str) -> AppServerResult<CustomerObject> {
        fetch_customer::handler(&self.secret_key, customer_id).await
    }

    async fn fetch_subscription(
        &self,
        subscription_id: &str,
    ) -> AppServerResult<SubscriptionObject> {
        fetch_subscription::handler(&self.secret_key, subscription_id).await
    }

    async fn send_usage_meter_event(
        &self,
        req: &send_usage_meter_event::RequestBody,
    ) -> AppServerResult {
        send_usage_meter_event::handler(&self.secret_key, req).await
    }
}
