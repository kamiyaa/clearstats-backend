use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserCredential {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: bool,
    pub locked: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct UserProfileBrief {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub icon_hash: Option<String>,
}

impl UserProfileBrief {
    pub fn new(
        username: String,
        first_name: String,
        last_name: String,
        icon_hash: Option<String>,
    ) -> Self {
        Self {
            username,
            first_name,
            last_name,
            icon_hash,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct UserProfileBriefBuilder {
    pub _username: Option<String>,
    pub _first_name: Option<String>,
    pub _last_name: Option<String>,
    pub _icon_hash: Option<String>,
}

impl UserProfileBriefBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn username(mut self, value: Option<String>) -> Self {
        self._username = value;
        self
    }
    pub fn first_name(mut self, value: Option<String>) -> Self {
        self._first_name = value;
        self
    }
    pub fn last_name(mut self, value: Option<String>) -> Self {
        self._last_name = value;
        self
    }
    pub fn icon_hash(mut self, value: Option<String>) -> Self {
        self._icon_hash = value;
        self
    }
    pub fn build(self) -> Option<UserProfileBrief> {
        match (self._username, self._first_name, self._last_name) {
            (Some(username), Some(first_name), Some(last_name)) => Some(UserProfileBrief {
                username,
                first_name,
                last_name,
                icon_hash: self._icon_hash,
            }),
            _ => None,
        }
    }
}
