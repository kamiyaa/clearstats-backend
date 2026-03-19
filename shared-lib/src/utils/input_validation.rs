pub fn validate_email(email: &str) -> bool {
    email.contains('@')
}

pub fn validate_username(s: &str) -> bool {
    for ch in s.chars() {
        if ch == '-' || ch == '_' || ch == '.' {
            continue;
        }
        if ch.is_uppercase() {
            return false;
        }
        if !ch.is_ascii_alphanumeric() {
            return false;
        }
    }
    true
}

pub fn validate_labspace_id(s: &str) -> bool {
    validate_username(s)
}
