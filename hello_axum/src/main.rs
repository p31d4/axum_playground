use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("<h1>Hello Axum</h1>") }),
    );

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
