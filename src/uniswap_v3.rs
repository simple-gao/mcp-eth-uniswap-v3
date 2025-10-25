use ethers::prelude::*;
use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::providers::{Provider, Http};
use serde_json::Value;
use anyhow::Result;
use std::sync::Arc;
use std::env;
use dotenv::dotenv;

pub async fn get_token_price(params: Value) -> Result<Value> {

    let pool_address = params.get("pool_address")
        .ok_or_else(|| anyhow::anyhow!("Missing pool_address"))?
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("pool_address must be a string"))?;

    dotenv().ok();
    let rpc_url = env::var("ETH_RPC_URL")?;

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);

    let abi_bytes = include_bytes!("abi/uniswap_v3_pool.json");
    let abi: Abi = serde_json::from_slice(abi_bytes)?;

    let pool_addr = pool_address.parse::<Address>()?;
    let contract = Contract::new(pool_addr, abi, provider.clone());


    let slot0: (U256, i32, u16, u16, u16, u8, bool) = contract.method::<_, _>("slot0", ())?.call().await?;

    let sqrt_price_x96 = slot0.0;
 
    let price = sqrt_price_x96.as_u128() as f64 / 2f64.powi(96);
    let price = price * price;

    Ok(serde_json::json!({ "price": price.to_string() }))
}

// 模拟 Uniswap V3 swap
pub async fn simulate_swap(_params: Value) -> Result<Value> {
    Ok(serde_json::json!({
        "estimated_out": "0.95",
        "gas_cost": "21000"
    }))
}
