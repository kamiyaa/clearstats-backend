use crate::{
    error::AppServerResult,
    integrations::gcp::{
        MockGoogleCloudClient,
        acl::{GoogleAuthClientTrait, generate_signed_url, sign_blob},
        no_mock_response_error,
    },
};

#[async_trait::async_trait]
impl<'a> GoogleAuthClientTrait for MockGoogleCloudClient<'a> {
    async fn sign_blob(
        &self,
        service_account_email: &str,
        payload: &[u8],
    ) -> AppServerResult<sign_blob::ResponseBody> {
        tracing::debug!(service_account_email, payload, "mock request");
        match self.responses.sign_blob.lock().await.pop_front() {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }

    async fn generate_signed_url(
        &self,
        req: &generate_signed_url::RequestBody,
    ) -> AppServerResult<String> {
        tracing::debug!(?req, "mock request");
        match self.responses.generate_signed_url.lock().await.pop_front() {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }
}
