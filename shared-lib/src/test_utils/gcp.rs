use std::sync::Arc;

use tokio::sync::RwLock;

use crate::integrations::gcp::{ServiceAccountAuthToken, ServiceAccountAuthTokenRaw};

pub fn generate_test_gcp_access_token_arc() -> Arc<RwLock<ServiceAccountAuthToken>> {
    Arc::new(RwLock::new(generate_test_gcp_access_token().into()))
}

pub fn generate_test_gcp_access_token() -> ServiceAccountAuthTokenRaw {
    ServiceAccountAuthTokenRaw {
        access_token: "Not a real token".to_string(),
        expires_in: 1234,
        token_type: "Random Token Type".to_string(),
    }
}
