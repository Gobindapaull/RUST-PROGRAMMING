use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Define a route
    let app = Router::new().route("/", get(|| async { "Hello from Rust on VPS 🚀" }));

    // Bind to 0.0.0.0:3000 so it’s reachable outside VPS
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    // Create TCP listener
    let listener = TcpListener::bind(addr).await.unwrap();

    // Serve the app
    axum::serve(listener, app).await.unwrap();
}

