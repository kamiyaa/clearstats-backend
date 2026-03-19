use serde::{Deserialize, Serialize};

use super::{
    CheckoutSessionObject, CustomerObject, InvoiceObject, LineItemObject, PaymentMethodObject,
    PlanObject, PriceObject, SubscriptionItemObject, SubscriptionObject,
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "object")]
pub enum StripeObject {
    #[serde(rename = "checkout.session")]
    CheckoutSession(Box<CheckoutSessionObject>),
    #[serde(rename = "customer")]
    Customer(Box<CustomerObject>),
    #[serde(rename = "invoice")]
    Invoice(Box<InvoiceObject>),
    #[serde(rename = "item")]
    Item(Box<ItemObject>),
    #[serde(rename = "line_item")]
    LineItem(Box<LineItemObject>),
    #[serde(rename = "list")]
    List(ListObject),
    #[serde(rename = "payment_method")]
    PaymentMethod(Box<PaymentMethodObject>),
    #[serde(rename = "plan")]
    Plan(Box<PlanObject>),
    #[serde(rename = "price")]
    Price(Box<PriceObject>),
    #[serde(rename = "subscription")]
    Subscription(Box<SubscriptionObject>),
    #[serde(rename = "subscription_item")]
    SubscriptionItem(Box<SubscriptionItemObject>),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ListObject {
    pub data: Vec<StripeObject>,
}

impl ListObject {
    pub fn new(data: Vec<StripeObject>) -> Self {
        Self { data }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ItemObject {
    pub id: String,
    pub price: PriceObject,
}
