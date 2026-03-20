use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorResponse {
    pub id: u64,
    pub name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub affiliation: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceResponse {
    pub id: u64,
    pub url: String,
    pub title: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttachmentResponse {
    pub id: u64,
    pub url: String,
    pub filename: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatisticResponse {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub sources: Vec<SourceResponse>,
    pub attachments: Vec<AttachmentResponse>,
    pub authors: Vec<AuthorResponse>,
    pub posted_by: UserResponse,
    pub upvotes: u64,
    pub downvotes: u64,
    pub user_vote: Option<i8>,
    pub question_count: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestionResponse {
    pub id: u64,
    pub statistic_id: u64,
    pub body: String,
    pub posted_by: UserResponse,
    pub upvotes: u64,
    pub downvotes: u64,
    pub user_vote: Option<i8>,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}
