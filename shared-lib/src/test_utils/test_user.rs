use crate::{
    database::{DatabaseInteger}, error::AppServerResult, types::{
        jwt::{self, UserClaims},
        user::UserProfileBrief,
    }
};

use super::jwt::generate_access_token;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TestUser {
    Alice,
    Bob,
    Charlie,
    Daniel,
    Edward,
    Frank,
    BlackHat,
}

impl TestUser {
    pub fn user_id(&self) -> DatabaseInteger {
        match self {
            Self::Alice => 1,
            Self::BlackHat => 7,
            Self::Bob => 2,
            Self::Charlie => 3,
            Self::Daniel => 4,
            Self::Edward => 5,
            Self::Frank => 6,
        }
    }

    pub fn username(&self) -> &str {
        match self {
            Self::Alice => "alice",
            Self::BlackHat => "blackhat",
            Self::Bob => "bob",
            Self::Charlie => "charlie",
            Self::Daniel => "daniel",
            Self::Edward => "edward",
            Self::Frank => "frank",
        }
    }

    pub fn first_name(&self) -> &str {
        match self {
            Self::Alice => "Alice",
            Self::BlackHat => "Blackhat",
            Self::Bob => "Bob",
            Self::Charlie => "Charlie",
            Self::Daniel => "Daniel",
            Self::Edward => "Edward",
            Self::Frank => "Frank",
        }
    }

    pub fn last_name(&self) -> &str {
        match self {
            Self::Alice => "Wonderland",
            Self::BlackHat => "Whitehat",
            Self::Bob => "Builder",
            Self::Charlie => "Brown",
            Self::Daniel => "Negreanu",
            Self::Edward => "Elric",
            Self::Frank => "Enstein",
        }
    }

    pub fn generate_jwt(&self, key: &[u8]) -> AppServerResult<String> {
        let user_id = self.user_id();
        let username = self.username();
        let first_name = self.first_name();
        let last_name = self.last_name();
        let user_claims = match self {
            Self::Alice => UserClaims {
                user_id,
                username: username.into(),
                first_name: first_name.into(),
                last_name: last_name.into(),
                verified: true,
                icon_hash: None,
            },
            Self::Bob => UserClaims {
                user_id,
                username: username.into(),
                first_name: first_name.into(),
                last_name: last_name.into(),
                verified: true,
                icon_hash: None,
            },
            Self::Charlie => UserClaims {
                user_id,
                username: username.into(),
                first_name: first_name.into(),
                last_name: last_name.into(),
                verified: true,
                icon_hash: None,
            },
            Self::Daniel => UserClaims {
                user_id,
                username: username.into(),
                first_name: first_name.into(),
                last_name: last_name.into(),
                verified: false,
                icon_hash: None,
            },
            Self::Edward => UserClaims {
                user_id,
                username: username.into(),
                first_name: first_name.into(),
                last_name: last_name.into(),
                verified: true,
                icon_hash: None,
            },
            Self::Frank => UserClaims {
                user_id,
                username: username.into(),
                first_name: first_name.into(),
                last_name: last_name.into(),
                verified: true,
                icon_hash: None,
            },
            Self::BlackHat => UserClaims {
                user_id,
                username: username.into(),
                first_name: first_name.into(),
                last_name: last_name.into(),
                verified: true,
                icon_hash: None,
            },
        };

        _generate_test_jwt(key, &user_claims)
    }

    pub fn generate_user_profile_brief(&self) -> UserProfileBrief {
        match self {
            Self::Alice => UserProfileBrief {
                username: "alice".to_string(),
                first_name: "Alice".to_string(),
                last_name: "Wonderland".to_string(),
                icon_hash: None,
            },
            Self::Bob => UserProfileBrief {
                username: "bob".into(),
                first_name: "Bob".into(),
                last_name: "Builder".into(),
                icon_hash: None,
            },
            Self::Charlie => UserProfileBrief {
                username: "charlie".into(),
                first_name: "Charlie".into(),
                last_name: "Brown".into(),
                icon_hash: None,
            },
            Self::Daniel => UserProfileBrief {
                username: "daniel".into(),
                first_name: "Daniel".into(),
                last_name: "Negreanu".into(),
                icon_hash: None,
            },
            Self::Edward => UserProfileBrief {
                username: "edward".into(),
                first_name: "Edward".into(),
                last_name: "Elric".into(),
                icon_hash: None,
            },
            Self::Frank => UserProfileBrief {
                username: "frank".into(),
                first_name: "Frank".into(),
                last_name: "Enstein".into(),
                icon_hash: None,
            },
            Self::BlackHat => UserProfileBrief {
                username: "frank".into(),
                first_name: "Frank".into(),
                last_name: "Enstein".into(),
                icon_hash: None,
            },
        }
    }
}

fn _generate_test_jwt(key: &[u8], user_claims: &UserClaims) -> AppServerResult<String> {
    let jwt_data = jwt::AccessToken {
        user: user_claims.clone(),
        exp: DatabaseInteger::MAX - 1,
    };

    let jwt_token = generate_access_token(key, &jwt_data)?;
    Ok(jwt_token)
}
