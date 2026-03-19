use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PriceObject {
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PlanObject {
    pub id: String,
    pub active: bool,
    pub product: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentMethodObject {
    pub id: String,
    pub customer: String,
    pub created: u64,
    pub status: String,
}
