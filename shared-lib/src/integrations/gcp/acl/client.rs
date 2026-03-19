use crate::{
    error::AppServerResult,
    integrations::gcp::{
        GoogleCloudClient,
        acl::{generate_signed_url, sign_blob},
    },
};

/// Trait describing what we can do with a Google Bucket Client
#[async_trait::async_trait]
pub trait GoogleAuthClientTrait {
    async fn sign_blob(
        &self,
        service_account_email: &str,
        payload: &[u8],
    ) -> AppServerResult<sign_blob::ResponseBody>;

    async fn generate_signed_url(
        &self,
        req: &generate_signed_url::RequestBody,
    ) -> AppServerResult<String>;
}

#[async_trait::async_trait]
impl GoogleAuthClientTrait for GoogleCloudClient {
    async fn sign_blob(
        &self,
        service_account_email: &str,
        payload: &[u8],
    ) -> AppServerResult<sign_blob::ResponseBody> {
        let req = sign_blob::RequestBody {
            service_account_email: service_account_email.to_string(),
            payload,
        };
        sign_blob::handler(&self.access_token, &req).await
    }

    async fn generate_signed_url(
        &self,
        req: &generate_signed_url::RequestBody,
    ) -> AppServerResult<String> {
        generate_signed_url::handler(&self.access_token, req).await
    }
}
