#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TestInstitution {
    Alice,
    Bob,
    Charlie,
    Edward,
    Frank,
}

impl TestInstitution {
    pub fn as_id(&self) -> u64 {
        match self {
            Self::Alice => 1000,
            Self::Bob => 2000,
            Self::Charlie => 3000,
            Self::Edward => 4000,
            Self::Frank => 5000,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Alice => "Alice Institution",
            Self::Bob => "Bob Institution",
            Self::Charlie => "Charlie Institution",
            Self::Edward => "Edward Institution",
            Self::Frank => "Frank Institution",
        }
    }

    pub fn name_string(&self) -> String {
        self.name().to_string()
    }
}

impl std::fmt::Display for TestInstitution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_id())
    }
}
