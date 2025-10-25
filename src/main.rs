use dotenv::dotenv;
use tracing_subscriber;

mod mcp_server;
mod eth_client;
mod uniswap_v3;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    mcp_server::start().await?;

    Ok(())
}
