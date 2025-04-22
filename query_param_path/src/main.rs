use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello/{name}", get(handler_hello_path));

    // Start Server
    // ------------------------------------------------------------------------
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!(">>> LISTENING on 127.0.0.1:3000\n");
    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
    // ------------------------------------------------------------------------
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

// Handler - Hello Path
// ----------------------------------------------------------------------------
async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
    println!(">>> {:<12} - handler_hello_path - {name:?}", "HANDLER");

    Html(format!("<h1>Hello {name}</h1>"))
}
