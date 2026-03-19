use serde::{Deserialize, Serialize};

use crate::{error::AppServerResult, test_utils::parse::str_to_u64};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GcpBucketObject {
    pub kind: String,
    pub id: String,
    #[serde(rename = "selfLink")]
    pub self_link: String,
    #[serde(rename = "mediaLink")]
    pub media_link: String,

    pub name: String,
    pub bucket: String,
    pub generation: String,
    pub metageneration: String,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "storageClass")]
    pub storage_class: String,
    pub size: String,
    #[serde(rename = "md5Hash")]
    pub md5_hash: String,
    pub crc32c: String,
    pub etag: String,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    pub updated: String,
    #[serde(rename = "timeStorageClassUpdated")]
    pub time_storage_class_updated: String,
}

impl GcpBucketObject {
    pub fn size_bytes(&self) -> AppServerResult<u64> {
        str_to_u64(&self.size)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GcpObjectListResponse {
    pub kind: String,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    pub prefixes: Option<Vec<String>>,
    pub items: Option<Vec<GcpBucketObject>>,
}

impl std::default::Default for GcpObjectListResponse {
    fn default() -> Self {
        Self {
            kind: "".into(),
            next_page_token: None,
            prefixes: None,
            items: None,
        }
    }
}
