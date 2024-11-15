pub fn validate_code(code: &str) -> bool {
    !code.is_empty() && code.chars().all(|c| c.is_ascii_digit())
}