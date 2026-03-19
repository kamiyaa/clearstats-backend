pub mod auth;
pub mod checkout;
pub mod customer;
pub mod subscription;
pub mod usage_meter;

pub mod mock_client;

mod stripe_client;
pub use stripe_client::*;

use crate::{
    error::AppServerResult,
    integrations::stripe::types::{CustomerObject, SubscriptionObject},
};

use super::types::ListObject;

pub const STRIPE_API_ENDPOINT: &str = "https://api.stripe.com";
pub const STRIPE_API_VERSION: &str = "2026-02-25.clover";

#[async_trait::async_trait]
pub trait StripeClientTrait {
    async fn create_customer(
        &self,
        req: &customer::create_customer::RequestBody,
    ) -> AppServerResult<customer::create_customer::ResponseBody>;

    async fn create_checkout(
        &self,
        req: &checkout::create_checkout::RequestBody,
    ) -> AppServerResult<checkout::create_checkout::ResponseBody>;

    async fn fetch_customer(&self, customer_id: &str) -> AppServerResult<CustomerObject>;

    async fn fetch_checkout_line_items(&self, checkout_id: &str) -> AppServerResult<ListObject>;

    async fn fetch_subscription(
        &self,
        subscription_id: &str,
    ) -> AppServerResult<SubscriptionObject>;

    async fn send_usage_meter_event(
        &self,
        req: &usage_meter::send_usage_meter_event::RequestBody,
    ) -> AppServerResult;
}
