use tower::make::Shared;

use app::{config::Config, service::new_service};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cfg = Config::from_default_sources()?;

    tracing::debug!("starting server with config: {:?}", cfg);

    axum::Server::bind(&cfg.listen_addr)
        .serve(Shared::new(new_service(&cfg)))
        .await?;

    Ok(())
}
