use serde::{Deserialize, Serialize};

use super::ListObject;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct InvoiceObject {
    pub id: String,
    pub customer: String,
    pub created: u64,
    pub status: String,
    pub hosted_invoice_url: String,
    pub invoice_pdf: String,
    pub lines: ListObject,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LineItemObject {
    pub id: String,
    pub pricing: Option<LineItemPricing>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LineItemPricing {
    pub price_details: PricingPriceDetails,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PricingPriceDetails {
    pub price: String,
    pub product: String,
}
