use std::net::SocketAddr;

use axum::{Router, ServiceExt};
use axum::response::Html;
use axum::routing::get;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();
    let app = Router::new()
        .route("/", get(root));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello,world</h1>")
}

