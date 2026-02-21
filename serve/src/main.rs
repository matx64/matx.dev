use axum::Router;
use std::path::Path;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    matx_dev::generate_website();

    let dist_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("dist");
    let app = Router::new().fallback_service(ServeDir::new(dist_path));

    let addr = "localhost:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    println!("🚀 Serving site at http://{}", addr);
    axum::serve(listener, app).await.expect("Server error");
}
