#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TestLab {
    Alice,
    Bob,
    Charlie,
    Edward,
    Frank,
}

impl TestLab {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Alice => "alice-lab",
            Self::Bob => "bob-lab",
            Self::Charlie => "charlie-lab",
            Self::Edward => "edward-lab",
            Self::Frank => "frank-lab",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Alice => "Alice Lab",
            Self::Bob => "Bob Lab",
            Self::Charlie => "Charlie Lab",
            Self::Edward => "Edward Lab",
            Self::Frank => "Frank Lab",
        }
    }

    pub fn name_string(&self) -> String {
        self.name().to_string()
    }
}

impl std::fmt::Display for TestLab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
