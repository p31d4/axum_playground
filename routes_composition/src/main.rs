use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello());

    // Start Server
    // ------------------------------------------------------------------------
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!(">>> LISTENING on 127.0.0.1:3000\n");
    axum::serve(listener, routes_all)
        .await
        .unwrap();
    // ------------------------------------------------------------------------
}

// Routes Hello
// ----------------------------------------------------------------------------
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hellopath/{name}", get(handler_hello_path))
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

// Handler - Hello Path
// ----------------------------------------------------------------------------
async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
    println!(">>> {:<12} - handler_hello_path - {name:?}", "HANDLER");

    Html(format!("<h1>Hello {name} (Path)</h1>"))
}
