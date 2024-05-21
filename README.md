# `SwapKit`

Swapkit’s SDK gives developers API access to a powerful suite of non-custodial, permissionless `DeFi` tools to interact with 5,500+ crypto assets across 14+ blockchains including Bitcoin, Ethereum, BNB Chain via `THORChain`, Chainflip and Maya Protocol.

## swapkit-rs

This is the Unofficial Rust bindings for the SwapKit API. It aims to provide a fully typeed client for the Swapkit API.

The client is rate limited to 1 request per second by default but this can be changed by creating a new `Configuration` object and passing it to the `Config::set_rate_limit_ms()` method.

## Supported Endpoints

* `get_chains` - Returns a list of all supported chains.
* `get_chains_with_details` - Returns a list of all supported chains with details.
* `get_gas_prices` - Returns a list of gas prices for all supported chains.
* `get_available_assets_for_pool` - ***Errors (Thorswap endpoint does not return a value.)***
* `get_available_lending_assets` - Returns a list of all available lending assets.
* `get_loans` - Returns a loan for a given address and asset.
* `get_supported_providers` - Returns a list of all supported providers.
* `get_request_a_swap_quote` - Returns a swap quote for a given swap request.
* `get_request_a_borrow_quote` - Returns a borrow quote for a given borrow request.
* `get_request_a_repay_quote` - Returns a repay quote for a given repay request.
* `get_minimum_amount_to_send_with_details` - Returns the minimum amount to send with details.
* `get_gas_history` - Returns the gas history for a given chain.
* `get_gas_rates` - Returns the gas rates for all chains.
* `get_currencies_with_details` - Returns a list of all supported currencies with details.
* `get_token_pair_exchange_rate` - Returns the exchange rate for a given token pair.
* `get_cached_prices` - Returns the cached prices for given tokens.
* `get_token_providers` - Returns a list of all token providers.
* `get_transaction_details` - *** Errors (Thorswap endpoint does not return a value.)***



## Basic Usage

```rust
use swapkit_rs::Swapkit;
use dotenv;
use swapkit_rs::Configuration;

#[tokio::main]
async fn main() {
    let swapkit_config = Configuration::new(None, dotenv::var("SWAPKIT_REFERER").unwrap().as_str(), dotenv::var("SWAPKIT_X_API_KEY").unwrap().as_str());
    let mut swapkit = Swapkit::new(swapkit_config);
    let supported_chains = swapkit.get_supported_chains().await.unwrap();

    assert_ne!(supported_chains.get_chains().len(), 0);
}
```
