use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use serde_json::{Value, json};
use std::sync::Arc;
use std::env;
use dotenv::dotenv;
use anyhow::Result;

pub async fn get_balance(params: Value) -> Result<Value> {
    let address_str = params.get("address")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing address"))?;

    dotenv().ok();
    let rpc_url = env::var("ETH_RPC_URL")?;
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);

    let address: Address = address_str.parse()?;
    let balance = provider.get_balance(address, None).await?;

    Ok(json!({"balance": balance.to_string()}))
}
