use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CheckoutSessionObject {
    pub id: String,
    pub created: u64,
    pub status: String,
    pub customer: String,
    pub subscription: Option<String>,
}
