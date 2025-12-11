mod server;
mod routes;
mod dto;
mod error;
mod templates;

use server::start_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_server().await
}

