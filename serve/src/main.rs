use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    matx_dev::main()?;

    let addr = &"127.0.0.1:1337";

    let app = Router::new().nest_service(
        "",
        get_service(ServeDir::new(format!(
            "{}/../dist",
            env!("CARGO_MANIFEST_DIR")
        ))),
    );

    println!("\n💻 Serving website at http://{}", &addr);
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
