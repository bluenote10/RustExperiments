use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(html_route))
        .route("/plain_string", get(plain_string_route))
        .route("/binary_route", get(binary_route));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn html_route() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn plain_string_route() -> &'static str {
    "Hello, World!"
}

async fn binary_route() -> &'static [u8] {
    &[0, 1, 2, 3]
}
