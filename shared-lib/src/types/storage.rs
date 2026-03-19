use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::error::ServerErrorResponse;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CloudService {
    Aws,
    Azure,
    Gcp,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PrivateStorageRegion {
    #[serde(rename = "north-america-1")]
    NorthAmerica1,
    #[serde(rename = "north-america-2")]
    NorthAmerica2,
}

impl PrivateStorageRegion {
    pub fn to_bucket_name(&self, prefix: &str) -> String {
        format!("{prefix}-private-{}", self.as_str())
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::NorthAmerica1 => "north-america-1",
            Self::NorthAmerica2 => "north-america-2",
        }
    }

    pub fn location(&self) -> &str {
        match self {
            Self::NorthAmerica1 => "Toronto, Canada",
            Self::NorthAmerica2 => "South Carolina, USA",
        }
    }

    pub fn cloud_service(&self) -> CloudService {
        match self {
            Self::NorthAmerica1 | Self::NorthAmerica2 => CloudService::Gcp,
        }
    }
}

impl TryFrom<&str> for PrivateStorageRegion {
    type Error = ServerErrorResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == Self::NorthAmerica1.as_str() {
            Ok(Self::NorthAmerica1)
        } else if value == Self::NorthAmerica2.as_str() {
            Ok(Self::NorthAmerica2)
        } else {
            Err(ServerErrorResponse::new(
                StatusCode::BAD_REQUEST,
                1234,
                "Unknown region".to_string(),
            ))
        }
    }
}
