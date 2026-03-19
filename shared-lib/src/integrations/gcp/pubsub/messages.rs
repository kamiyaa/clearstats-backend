use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PubSubMessageInner {
    #[serde(default)]
    pub attributes: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PubSubMessage {
    pub message: PubSubMessageInner,
    pub subscription: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendVerificationEmailMessage {
    pub email_address: String,
    pub verification_code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ModifyUserBadgesActions {
    #[serde(rename = "add_badge")]
    AddBadge,
    #[serde(rename = "remove_badge")]
    RemoveBadge,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModifyUserBadges {
    pub action: ModifyUserBadgesActions,
    pub username: String,
    pub badge_id: String,
}
