use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CustomerObject {
    pub id: String,
    pub object: String,
    pub address: Option<String>,
    pub balance: u64,
    pub created: u64,
    pub currency: Option<String>,
    pub email: String,
    pub name: String,
}
