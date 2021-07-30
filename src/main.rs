use axum::prelude::*;
use std::net::SocketAddr;
use http::StatusCode;
use axum::response::{Html, Json};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = route("/", get(root))
        .route("/html", get(html))
        .route("/json", get(json));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

// `Html` gives a content-type of `text/html`
async fn html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

