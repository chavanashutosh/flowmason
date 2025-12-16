mod server;

use flowmason_api::routes;
use server::start_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_server().await
}

