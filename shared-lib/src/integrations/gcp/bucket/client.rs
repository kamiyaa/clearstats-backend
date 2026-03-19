use crate::{
    error::AppServerResult,
    integrations::gcp::{GoogleCloudClient, bucket},
};

/// Trait describing what we can do with a Google Bucket Client
#[async_trait::async_trait]
pub trait GoogleBucketClientTrait {
    async fn insert_object(
        &self,
        bucket_name: &str,
        file_name: &str,
        content_type: Option<&str>,
        obj: Vec<u8>,
        generation: Option<&str>,
    ) -> AppServerResult<bucket::insert_object::ResponseBody>;

    async fn fetch_objects(
        &self,
        bucket_name: &str,
        encoded_path: &str,
    ) -> AppServerResult<bucket::GcpObjectListResponse>;

    async fn fetch_object_metadata(
        &self,
        bucket_name: &str,
        encoded_path: &str,
    ) -> AppServerResult<bucket::GcpBucketObject>;

    async fn delete_object(&self, bucket_name: &str, encoded_path: &str) -> AppServerResult;
}

#[async_trait::async_trait]
impl GoogleBucketClientTrait for GoogleCloudClient {
    async fn insert_object(
        &self,
        bucket_name: &str,
        file_name: &str,
        content_type: Option<&str>,
        obj: Vec<u8>,
        generation: Option<&str>,
    ) -> AppServerResult<bucket::insert_object::ResponseBody> {
        bucket::insert_object::handler(
            &self.access_token,
            bucket_name,
            file_name,
            content_type,
            obj,
            generation,
        )
        .await
    }

    async fn fetch_objects(
        &self,
        bucket_name: &str,
        encoded_path: &str,
    ) -> AppServerResult<bucket::GcpObjectListResponse> {
        bucket::fetch_object_list_files(&self.access_token, bucket_name, encoded_path).await
    }

    async fn fetch_object_metadata(
        &self,
        bucket_name: &str,
        encoded_path: &str,
    ) -> AppServerResult<bucket::GcpBucketObject> {
        bucket::fetch_metadata::handler(&self.access_token, bucket_name, encoded_path).await
    }

    async fn delete_object(&self, bucket_name: &str, encoded_path: &str) -> AppServerResult {
        bucket::delete_object::handler(&self.access_token, bucket_name, encoded_path).await
    }
}
