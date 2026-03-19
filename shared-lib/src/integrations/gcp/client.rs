#[derive(Clone, Debug)]
pub struct GoogleCloudClient {
    pub access_token: String,
}

impl GoogleCloudClient {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
}
