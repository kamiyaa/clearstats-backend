use crate::types::storage::PrivateStorageRegion;

pub fn generate_public_bucket_name(prefix: &str) -> String {
    format!("{prefix}-indaggo-public")
}

pub fn generate_private_bucket_name(prefix: &str, region: &PrivateStorageRegion) -> String {
    format!("{prefix}-indaggo-private-{}", region.as_str())
}
