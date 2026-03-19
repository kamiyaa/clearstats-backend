use crate::{
    error::AppServerResult,
    integrations::gcp::{
        MockGoogleCloudClient,
        bucket::{self, GoogleBucketClientTrait},
        no_mock_response_error,
    },
};

#[async_trait::async_trait]
impl<'a> GoogleBucketClientTrait for MockGoogleCloudClient<'a> {
    async fn insert_object(
        &self,
        bucket_name: &str,
        file_name: &str,
        content_type: Option<&str>,
        _obj: Vec<u8>,
        _generation: Option<&str>,
    ) -> AppServerResult<bucket::insert_object::ResponseBody> {
        tracing::debug!(bucket_name, file_name, content_type, "mock request");
        match self.responses.insert_object.lock().await.pop_front() {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }

    async fn fetch_objects(
        &self,
        bucket_name: &str,
        encoded_path: &str,
    ) -> AppServerResult<bucket::GcpObjectListResponse> {
        tracing::debug!(bucket_name, encoded_path, "mock request");
        match self.responses.fetch_objects.lock().await.pop_front() {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }

    async fn fetch_object_metadata(
        &self,
        bucket_name: &str,
        encoded_path: &str,
    ) -> AppServerResult<bucket::GcpBucketObject> {
        tracing::debug!(bucket_name, encoded_path, "mock request");
        match self
            .responses
            .fetch_object_metadata
            .lock()
            .await
            .pop_front()
        {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }

    async fn delete_object(&self, bucket_name: &str, encoded_path: &str) -> AppServerResult {
        tracing::debug!(bucket_name, encoded_path, "mock request");
        match self.responses.delete_object.lock().await.pop_front() {
            Some(response) => {
                tracing::debug!(?response, "mock response");
                response
            }
            None => Err(no_mock_response_error()),
        }
    }
}
