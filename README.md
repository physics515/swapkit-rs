# `SwapKit`

Swapkit’s SDK gives developers API access to a powerful suite of non-custodial, permissionless `DeFi` tools to interact with 5,500+ crypto assets across 14+ blockchains including Bitcoin, Ethereum, BNB Chain via `THORChain`, Chainflip and Maya Protocol.

## swapkit-rs

This is the Unofficial Rust bindings for the SwapKit API. It aims to provide a fully typed client for the SwapKit API.

The client is rate limited to 1 request per second by default but this can be changed by creating a new `Configuration` object and passing it to the `Config::set_rate_limit_ms()` method.

## Supported Endpoints

> **Upstream status (probed 2026-09-24).** Every path on `https://api.thorswap.net/` — the base
> URL this crate targets by default — answers **HTTP 404**, including `/aggregator/chains`,
> `/aggregator/tokens/quote` and `/tokenlist/utils/providers`. The current SwapKit API is a v3
> REST surface at `https://api.swapkit.dev` (live and auth-gated: `/providers` and `/tokens`
> answer 401; the OpenAPI definition is served at <https://api.swapkit.dev/docs/json>). **Until
> the migration lands, expect the endpoints below to fail against the default base URL.**
> `Configuration::set_base_url` lets you point the client elsewhere in the meantime.

* `get_supported_chains` - Returns a list of all supported chains.
* `get_chains_with_details` - Returns a list of all supported chains with details.
* `get_gas_prices` - Returns a list of gas prices for all supported chains.
* `get_available_assets_for_pool` - Returns the assets available for a lending pool.
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
* `get_transaction_details` - Placeholder; the response is not modelled and is discarded.

## Basic Usage

```rust
use swapkit_rs::{Configuration, Swapkit};

#[tokio::main]
async fn main() {
    let referer = std::env::var("SWAPKIT_REFERER").expect("SWAPKIT_REFERER");
    let x_api_key = std::env::var("SWAPKIT_X_API_KEY").expect("SWAPKIT_X_API_KEY");

    let swapkit_config = Configuration::new(None, &referer, &x_api_key);
    let mut swapkit = Swapkit::new(swapkit_config);
    let supported_chains = swapkit.get_supported_chains().await.unwrap();

    assert_ne!(supported_chains.get_chains().len(), 0);
}
```

## Minimum supported Rust version

**Rust 1.83.** Verified by building and running the full test suite on 1.83.0; 1.82 does not
compile the crate, because the `const fn` setters take `&mut self` and `const_mut_refs` was
stabilised in 1.83.

Nightly is needed only to run `cargo fmt`, because `rustfmt.toml` uses unstable options.
