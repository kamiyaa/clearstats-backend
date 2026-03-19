pub enum AppTopics {
    SendEmailVerification,
}

impl std::convert::AsRef<str> for AppTopics {
    fn as_ref(&self) -> &str {
        match self {
            Self::SendEmailVerification => "send-verification-email",
        }
    }
}
