use serde::{Deserialize, Serialize};

use super::{ListObject, PlanObject, PriceObject};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionObject {
    pub id: String,
    pub customer: String,
    pub created: u64,
    pub status: String,
    pub items: ListObject,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionItemObject {
    pub id: String,
    pub created: u64,
    pub plan: PlanObject,
    pub price: PriceObject,
}
