use serde_json::{json, Value};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::eth_client;
use crate::uniswap_v3;

pub async fn start() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin).lines();
    let mut writer = stdout;

    println!("MCP Server started. Listening for JSON requests...");

    while let Some(line) = reader.next_line().await? {
        let request: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => {
                let err = json!({"error": "Invalid JSON"});
                writer.write_all(format!("{}\n", err).as_bytes()).await?;
                continue;
            }
        };

        let method = request.get("method").and_then(|v| v.as_str()).unwrap_or("");
        let params = request.get("params").cloned().unwrap_or(json!({}));

        let response = match method {
            "get_balance" => eth_client::get_balance(params).await?,
            "get_token_price" => uniswap_v3::get_token_price(params).await?,
            "swap_tokens" => uniswap_v3::simulate_swap(params).await?,
            _ => json!({"error": format!("Unknown method: {}", method)}),
        };

        writer.write_all(format!("{}\n", response).as_bytes()).await?;
    }

    Ok(())
}
