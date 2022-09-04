use tower::make::Shared;

use app::{config::Config, service};

use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cfg = Arc::new(Config::from_default_sources()?);

    tracing::info!("starting server with config: {:#?}", cfg);

    let server = axum::Server::bind(&cfg.listen_addr).serve(Shared::new(service::new(cfg)));

    // Run until we receive a shutdown signal
    if let Err(e) = server.with_graceful_shutdown(shutdown_signal()).await {
        tracing::error!("server error: {}", e);
        std::process::exit(1);
    }

    tracing::info!("server stopped");

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");

    tracing::info!("server received shutdown signal");
}
