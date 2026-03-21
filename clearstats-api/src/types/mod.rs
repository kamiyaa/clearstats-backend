use serde::{Deserialize, Serialize};
use shared_lib::database::DatabaseInteger;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: DatabaseInteger,
    pub username: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorResponse {
    pub id: DatabaseInteger,
    pub name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub affiliation: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceResponse {
    pub id: DatabaseInteger,
    pub url: String,
    pub title: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttachmentResponse {
    pub id: DatabaseInteger,
    pub url: String,
    pub filename: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatisticResponse {
    pub id: DatabaseInteger,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub sources: Vec<SourceResponse>,
    pub attachments: Vec<AttachmentResponse>,
    pub authors: Vec<AuthorResponse>,
    pub posted_by: UserResponse,
    pub upvotes: DatabaseInteger,
    pub downvotes: DatabaseInteger,
    pub user_vote: Option<i8>,
    pub question_count: DatabaseInteger,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestionResponse {
    pub id: DatabaseInteger,
    pub statistic_id: DatabaseInteger,
    pub body: String,
    pub posted_by: UserResponse,
    pub upvotes: DatabaseInteger,
    pub downvotes: DatabaseInteger,
    pub user_vote: Option<i8>,
    pub created_at: String,
}
