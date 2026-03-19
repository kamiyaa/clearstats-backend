use std::{collections::VecDeque, sync::Arc};

use reqwest::StatusCode;
use tokio::sync::Mutex;

use crate::{
    error::{AppServerResult, ServerErrorResponse},
    integrations::stripe::types::{CustomerObject, ListObject, SubscriptionObject},
};

use super::{StripeClientTrait, checkout, customer, usage_meter};

type RequestList<T> = Arc<Mutex<Vec<T>>>;
type ResponseList<T> = Arc<Mutex<VecDeque<AppServerResult<T>>>>;

#[derive(Clone, Debug, Default)]
pub struct MockClientRequests {
    pub create_customer: RequestList<customer::create_customer::RequestBody>,
    pub create_checkout: RequestList<checkout::create_checkout::RequestBody>,
    pub fetch_customer: RequestList<String>,
    pub fetch_checkout_line_items: RequestList<String>,
    pub send_usage_meter_event: RequestList<usage_meter::send_usage_meter_event::RequestBody>,
}

#[derive(Clone, Debug, Default)]
pub struct MockClientResponses {
    pub create_customer: ResponseList<customer::create_customer::ResponseBody>,
    pub create_checkout: ResponseList<checkout::create_checkout::ResponseBody>,
    pub get_checkout_line_items: ResponseList<ListObject>,
    pub get_subscription: ResponseList<SubscriptionObject>,
    pub fetch_customer: ResponseList<CustomerObject>,
    pub send_usage_meter_event: ResponseList<()>,
}

#[derive(Clone, Debug, Default)]
pub struct MockStripeClient {
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
impl StripeClientTrait for MockStripeClient {
    async fn create_customer(
        &self,
        req: &customer::create_customer::RequestBody,
    ) -> AppServerResult<customer::create_customer::ResponseBody> {
        tracing::debug!(?req, "mock request");
        self.requests.create_customer.lock().await.push(req.clone());
        match self.responses.create_customer.lock().await.pop_front() {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }

    async fn create_checkout(
        &self,
        req: &checkout::create_checkout::RequestBody,
    ) -> AppServerResult<checkout::create_checkout::ResponseBody> {
        tracing::debug!(?req, "mock request");
        self.requests.create_checkout.lock().await.push(req.clone());
        match self.responses.create_checkout.lock().await.pop_front() {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }

    async fn fetch_checkout_line_items(&self, checkout_id: &str) -> AppServerResult<ListObject> {
        tracing::debug!(?checkout_id, "mock request");
        self.requests
            .fetch_checkout_line_items
            .lock()
            .await
            .push(checkout_id.to_string());
        match self
            .responses
            .get_checkout_line_items
            .lock()
            .await
            .pop_front()
        {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }

    async fn fetch_customer(&self, customer_id: &str) -> AppServerResult<CustomerObject> {
        tracing::debug!(?customer_id, "mock request");
        self.requests
            .fetch_customer
            .lock()
            .await
            .push(customer_id.to_string());
        match self.responses.fetch_customer.lock().await.pop_front() {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }

    async fn fetch_subscription(
        &self,
        subscription_id: &str,
    ) -> AppServerResult<SubscriptionObject> {
        tracing::debug!(?subscription_id, "mock request");
        self.requests
            .fetch_checkout_line_items
            .lock()
            .await
            .push(subscription_id.to_string());
        match self.responses.get_subscription.lock().await.pop_front() {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }

    async fn send_usage_meter_event(
        &self,
        req: &usage_meter::send_usage_meter_event::RequestBody,
    ) -> AppServerResult {
        tracing::debug!(?req, "mock request");
        self.requests
            .send_usage_meter_event
            .lock()
            .await
            .push(req.clone());
        match self
            .responses
            .send_usage_meter_event
            .lock()
            .await
            .pop_front()
        {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }
}
