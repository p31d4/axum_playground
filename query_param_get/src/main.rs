use axum::{
    Router,
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

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

// Handler - Hello
// ----------------------------------------------------------------------------
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!(">>> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("p31d4");
    Html(format!("<h1>Hello {name}</h1>"))
}
