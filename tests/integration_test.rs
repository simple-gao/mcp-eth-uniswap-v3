use dotenv::dotenv;
use std::env;
use serde_json::json;
use mcp_eth_uniswap_v3::{get_balance, get_token_price, simulate_swap};

#[tokio::test]
async fn test_all() {
    dotenv().ok();

    let address = env::var("TEST_ETH_ADDRESS")
        .expect("Please set TEST_ETH_ADDRESS in your .env file");
    let params_balance = json!({ "address": address });
    let balance = get_balance(params_balance).await.unwrap();
    println!("Balance result: {}", balance);

    let pool_address = "0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8";
    let params_price = json!({ "pool_address": pool_address });
    let price = get_token_price(params_price).await.unwrap();
    println!("Token price result: {}", price);

    let swap_params = json!({
        "token_in": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
        "token_out": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        "amount_in": "1000000000000000000"
    });
    let swap_result = simulate_swap(swap_params).await.unwrap();
    println!("Simulated swap result: {}", swap_result);
}
