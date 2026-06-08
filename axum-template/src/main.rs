use std::net::SocketAddr;
use tracing::info;

use crate::config::AppConfig;

mod config;
mod errors;
mod middleware;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    middleware::tracing::init();

    let cfg = AppConfig::load()?;
    let addr: SocketAddr = cfg.socket_addr();

    let app = routes::build_router(state::AppState::new(cfg));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(%addr, "server listening");

    axum::serve(listener, app).await?;
    Ok(())
}