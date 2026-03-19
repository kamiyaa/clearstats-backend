use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct PublishMessageResponse {
    #[serde(rename = "messageIds")]
    pub message_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PublishMessageRequest<T: Serialize> {
    pub messages: Vec<T>,
}
