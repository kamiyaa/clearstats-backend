#[derive(Clone, Debug)]
pub enum TestInstitutionRole {
    Admin,
    User,
}

impl TestInstitutionRole {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Admin => "Institution Admin",
            Self::User => "Institution User",
        }
    }
}

impl std::fmt::Display for TestInstitutionRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Debug)]
pub enum TestLabspaceRole {
    LabAdmin,
    LabManager,
    LabUser,
}

impl TestLabspaceRole {
    pub fn as_str(&self) -> &str {
        match self {
            Self::LabAdmin => "Lab Admin",
            Self::LabManager => "Lab Manager",
            Self::LabUser => "Lab User",
        }
    }
}

impl std::fmt::Display for TestLabspaceRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
