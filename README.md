# MCP ETH Uniswap V3 Tools

一个基于 Rust 的以太坊工具集合，支持：

- 获取 ETH / ERC20 地址余额
- 获取 ERC20 代币价格（通过 Uniswap V3 池）
- 模拟 Uniswap V3 代币 Swap

---

## 目录结构

mcp-eth-uniswap-v3/
├── Cargo.toml
├── .env
├── src/
│ ├── main.rs
│ ├── lib.rs
│ ├── eth_client.rs
│ ├── uniswap_v3.rs
│ ├── mcp_server.rs
│ └── abi/
│ └── erc20.json
├── tests/
│ └── integration_test.rs
└── README.md


---

## 环境变量配置

在项目根目录创建 `.env` 文件：

```text
# Ethereum RPC URL (Alchemy)
ETH_RPC_URL=https://eth-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_KEY

# 用于交易签名（模拟或真实交易）
PRIVATE_KEY=0xYourPrivateKey

# Ethereum 主网 Chain ID
CHAIN_ID=1

# 测试用地址
TEST_ETH_ADDRESS=0xYourEthereumAddress


# 构建项目
cargo build

# 运行主程序
cargo run

cargo test --test integration_test -- --nocapture
