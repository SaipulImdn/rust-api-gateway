use crate::domain::validation;

pub fn process_code(code: &str) -> Result<&str, &'static str> {
    if validation::validate_code(code) {
        Ok("Code is valid and processed.")
    } else {
        Err("Invalid code provided.")
    }
}
