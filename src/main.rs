use axum::prelude::*;
use std::net::SocketAddr;
use http::StatusCode;
use axum::response::{Html, Json};
use axum::{extract::{UrlParams}, routing::nest, service::ServiceExt};
use serde_json::{json, Value};
use tower_http::{services::ServeDir};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // static assets handler
    let static_handle = ServeDir::new("./static").handle_error(|error: std::io::Error| {
        Ok::<_, std::convert::Infallible>((StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {}", error)))
    });

    let app = nest("/", axum::service::get(static_handle))
        .route("/", get(index))
        .route("/html", get(html))
        .route("/user/:id", get(user))
        .route("/json", get(json));

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    hyper::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

// `Html` gives a content-type of `text/html`
async fn html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: u32,
    pub name: String,
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

async fn user(UrlParams((id, )): UrlParams<(u32, )>) -> Json<Person> {
    Json(Person { id, name: "linux_china".to_string() })
}

