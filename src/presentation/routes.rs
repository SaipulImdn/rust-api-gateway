use axum::{Json, extract::Form};
use serde::Deserialize;
use crate::use_cases::process_code;

#[derive(Deserialize)]
pub struct CodePayload {
    code: String,
}

pub async fn validate_code_handler(Form(payload): Form<CodePayload>) -> Json<&'static str> {
    match process_code(&payload.code) {
        Ok(message) => Json(message),
        Err(err) => Json(err),
    }
}