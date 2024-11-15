mod domain;
mod use_cases;
mod infrastructure;
mod presentation;

use infrastructure::http_server;
use axum::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = http_server::create_server().await;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
