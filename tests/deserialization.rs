//! Offline deserialization tests over pinned JSON fixtures.
//!
//! Every other test in this crate calls the live `SwapKit` API with a real key, so a red run there
//! could mean the network was down, the key was wrong, *or* the code was broken. These tests need
//! neither the network nor a credential: they are the only ones that fail if and only if the
//! deserialization is wrong.
//!
//! The fixtures under `tests/fixtures/` are the response shapes this crate has already had to be
//! fixed for — the git history is a string of gas-price and cached-price deserialization repairs
//! (`584cfaa`, `25e0381`, `53dd10a`, `235b692`, `43f9b38`). Each one is pinned here so the same
//! shape cannot regress silently.

use std::str::FromStr;

use rust_decimal::Decimal;
use swapkit_rs::{CachedPrice, GasPrices, SupportedChains};

fn fixture(name: &str) -> String {
	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/").to_string() + name;
	std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("could not read fixture {path}: {e}"))
}

fn dec(s: &str) -> Decimal {
	Decimal::from_str(s).expect("fixture decimal")
}

#[test]
fn gas_prices_deserialize() {
	let gas_prices: GasPrices = serde_json::from_str(&fixture("gas_prices.json")).expect("gas_prices.json");
	let prices = gas_prices.get_gas_prices();

	assert_eq!(prices.len(), 2);

	let rune = &prices[0];
	assert_eq!(rune.get_asset(), "THOR.RUNE");
	assert_eq!(rune.get_units(), "tor");
	assert_eq!(rune.get_chain_id(), "thorchain-mainnet-v1");
	assert_eq!(rune.get_gas(), &dec("2000000"));
	assert_eq!(rune.get_gas_asset(), &dec("0.02"));

	let btc = &prices[1];
	assert_eq!(btc.get_asset(), "BTC.BTC");
	assert_eq!(btc.get_gas(), &dec("12"));
	assert_eq!(btc.get_gas_asset(), &dec("0.00000012"));
}

/// Pins the contract of `deserialize_rust_decimal_from_anything`, which is what every past
/// gas-price fix actually changed: a decimal field may arrive as a number, as a numeric string, as
/// `null`, as `""`, or as one of the textual sentinels `"NULL"` / `"Infinity"` / `"inf"`. Every
/// non-numeric form must land on `Decimal::ZERO` rather than failing the whole response.
#[test]
fn gas_prices_tolerate_every_shape_upstream_has_sent() {
	let gas_prices: GasPrices = serde_json::from_str(&fixture("gas_prices_edge_cases.json")).expect("gas_prices_edge_cases.json");
	let prices = gas_prices.get_gas_prices();

	assert_eq!(prices.len(), 4);

	// numeric strings parse
	assert_eq!(prices[0].get_gas(), &dec("13.5"));
	assert_eq!(prices[0].get_gas_asset(), &dec("0.0000000135"));

	// JSON null -> zero
	assert_eq!(prices[1].get_gas(), &Decimal::ZERO);
	assert_eq!(prices[1].get_gas_asset(), &Decimal::ZERO);

	// empty string -> zero; "Infinity" -> zero
	assert_eq!(prices[2].get_gas(), &Decimal::ZERO);
	assert_eq!(prices[2].get_gas_asset(), &Decimal::ZERO);

	// "NULL" and "inf" are case-insensitive sentinels -> zero
	assert_eq!(prices[3].get_gas(), &Decimal::ZERO);
	assert_eq!(prices[3].get_gas_asset(), &Decimal::ZERO);
}

#[test]
fn supported_chains_deserialize() {
	let chains: SupportedChains = serde_json::from_str(&fixture("supported_chains.json")).expect("supported_chains.json");

	assert_eq!(chains.get_chains().len(), 10);
	assert_eq!(chains.get_id("BTC").map(String::as_str), Some("bitcoin"));
	assert_eq!(chains.get_id("ETH").map(String::as_str), Some("1"));
	assert_eq!(chains.get_id("NOPE"), None);
}

#[test]
fn cached_prices_deserialize() {
	let prices: Vec<CachedPrice> = serde_json::from_str(&fixture("cached_prices.json")).expect("cached_prices.json");

	assert_eq!(prices.len(), 2);

	let btc = &prices[0];
	assert_eq!(btc.get_identifier(), "BTC.BTC");
	assert_eq!(btc.get_provider().as_deref(), Some("thorchain"));
	assert_eq!(btc.get_price_usd(), Some(dec("62744.82080390614")));
	assert_eq!(btc.get_timestamp(), Some(dec("1710352724180")));
	assert_eq!(btc.get_cg().get_name().as_deref(), Some("Bitcoin"));

	// A price entry may arrive with no provider, an empty `cg` object and a null price. This is
	// the shape `43f9b38` ("prevent serde misses on cached prices") had to accommodate.
	let sparse = &prices[1];
	assert_eq!(sparse.get_provider().as_deref(), None);
	assert_eq!(sparse.get_price_usd(), None);
	assert_eq!(sparse.get_timestamp(), None);
	assert_eq!(sparse.get_cg().get_name().as_deref(), None);
}
