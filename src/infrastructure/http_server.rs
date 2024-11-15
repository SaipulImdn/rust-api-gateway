use axum::{Router, routing::post};
use crate::presentation::routes;

pub async fn create_server() -> Router {
    Router::new().route("/validate", post(routes::validate_code_handler))
}