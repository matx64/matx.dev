use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new().fallback_service(ServeDir::new(format!(
        "{}/../dist",
        env!("CARGO_MANIFEST_DIR")
    )));

    let addr = "localhost:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Serving site at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}