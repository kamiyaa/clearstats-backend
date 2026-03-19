use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppServerResult,
    integrations::stripe::client::auth::create_authenticated_request,
    utils::{request::reqwest_send_to_server_error, response::try_parse_json_response},
};

use super::super::STRIPE_API_ENDPOINT;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum CheckoutMode {
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "setup")]
    Setup,
    #[serde(rename = "subscription")]
    Subscription,
}

impl CheckoutMode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Payment => "payment",
            Self::Setup => "setup",
            Self::Subscription => "subscription",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct LineItem {
    pub price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RequestBody {
    pub customer: String,
    pub success_url: String,
    pub mode: CheckoutMode,
    pub line_items: Vec<LineItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    pub id: String,
    pub url: String,
}

pub async fn handler(api_key: &str, req: &RequestBody) -> AppServerResult<ResponseBody> {
    let mut form_data: HashMap<String, String> = HashMap::new();
    form_data.insert("customer".into(), req.customer.clone());
    form_data.insert("success_url".into(), req.success_url.clone());
    form_data.insert("mode".into(), req.mode.as_str().to_string());

    for (i, item) in req.line_items.iter().enumerate() {
        form_data.insert(format!("line_items[{i}][price]"), item.price.clone());
        if let Some(quantity) = item.quantity {
            form_data.insert(
                format!("line_items[{i}][quantity]"),
                format!("{}", quantity),
            );
        }
    }

    let api_url = format!("{STRIPE_API_ENDPOINT}/v1/checkout/sessions");
    let resp = create_authenticated_request(api_key, Method::POST, &api_url)
        .form(&form_data)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;

    try_parse_json_response(resp).await
}
