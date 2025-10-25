// src/lib.rs
pub mod eth_client;
pub mod uniswap_v3;
pub mod mcp_server;

// 将 get_balance 暴露出来给测试用
pub use eth_client::get_balance;
pub use uniswap_v3::get_token_price;
pub use uniswap_v3::simulate_swap;
