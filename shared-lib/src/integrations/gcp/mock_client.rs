use std::{collections::VecDeque, sync::Arc};

use reqwest::StatusCode;
use tokio::sync::Mutex;

use crate::{
    error::{AppServerResult, ServerErrorResponse},
    integrations::gcp::{acl, bucket},
};

type RequestList<T> = Arc<Mutex<Vec<T>>>;
type ResponseList<T> = Arc<Mutex<VecDeque<AppServerResult<T>>>>;

#[derive(Clone, Debug, Default)]
pub struct MockClientRequests<'a> {
    pub insert_object: RequestList<bucket::insert_object::ResponseBody>,
    pub fetch_objects: RequestList<bucket::GcpObjectListResponse>,
    pub delete_object: RequestList<AppServerResult>,

    pub generate_signed_url: RequestList<acl::generate_signed_url::RequestBody>,
    pub sign_blob: RequestList<acl::sign_blob::RequestBody<'a>>,
}

#[derive(Clone, Debug, Default)]
pub struct MockClientResponses {
    pub insert_object: ResponseList<bucket::insert_object::ResponseBody>,
    pub fetch_objects: ResponseList<bucket::GcpObjectListResponse>,
    pub fetch_object_metadata: ResponseList<bucket::GcpBucketObject>,
    pub delete_object: ResponseList<()>,

    pub generate_signed_url: ResponseList<String>,
    pub sign_blob: ResponseList<acl::sign_blob::ResponseBody>,
}

#[derive(Clone, Debug, Default)]
pub struct MockGoogleCloudClient<'a> {
    pub requests: MockClientRequests<'a>,
    pub responses: MockClientResponses,
}

pub fn no_mock_response_error() -> ServerErrorResponse {
    ServerErrorResponse::new(
        StatusCode::IM_A_TEAPOT,
        1000,
        "No mock response set".to_string(),
    )
}
