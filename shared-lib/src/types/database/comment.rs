use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::super::user::UserProfileBrief;

#[derive(Clone, Debug, FromRow)]
pub struct SqlComment {
    pub id: u64,

    pub content: String,

    pub created_at: u64,
    pub updated_at: u64,

    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub icon_hash: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct JsonComment {
    pub id: u64,
    pub content: String,
    pub created_by: UserProfileBrief,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<SqlComment> for JsonComment {
    fn from(value: SqlComment) -> Self {
        Self {
            id: value.id,
            content: value.content,
            created_by: UserProfileBrief {
                username: value.username,
                first_name: value.first_name,
                last_name: value.last_name,
                icon_hash: value.icon_hash,
            },
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
