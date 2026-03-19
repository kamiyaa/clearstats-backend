pub mod acl;
pub mod bucket;
pub mod pubsub;
pub mod util;

mod auth;
mod client;
mod mock_client;
pub use auth::*;
pub use client::*;
pub use mock_client::*;

pub const GCP_STORAGE_API_URL: &str = "https://storage.googleapis.com";

pub const CLOUD_RUN_AUTH_SERVER: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
