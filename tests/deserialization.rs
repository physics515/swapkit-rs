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
use swapkit_rs::{CachedPrice, ChainsWithDetails, ExchangeRate, GasHistory, GasPrices, Provider, SupportedChains, SupportedProviders};

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

#[test]
fn chains_with_details_deserialize() {
	let chains: ChainsWithDetails = serde_json::from_str(&fixture("chains_with_details.json")).expect("chains_with_details.json");
	let chains = chains.get_chains();

	assert_eq!(chains.len(), 2);

	let avax = &chains[0];
	assert_eq!(avax.get_chain(), "AVAX");
	// `chainId` arrives as a JSON number here and as a string in the second entry; both must land
	// on the same `Option<String>`.
	assert_eq!(avax.get_chain_id().as_deref(), Some("43114"));
	assert_eq!(avax.get_display_name().as_deref(), Some("Avalanche"));
	assert_eq!(avax.get_status(), "active");
	assert!(avax.get_evm());
	assert!(avax.get_mainnet());
	assert_eq!(avax.get_average_block_time(), &dec("3000"));
	assert_eq!(avax.get_default_decimals(), &Some(dec("18")));
	// `gasRate` is explicitly null upstream and must not fail the whole chain list.
	assert_eq!(avax.get_gas_rate(), None);
	assert_eq!(avax.get_providers().as_ref().map(Vec::len), Some(2));
	assert!(avax.get_gas_asset().is_some());

	// Every optional field genuinely absent: the shape a sparse chain entry arrives in.
	let btc = &chains[1];
	assert_eq!(btc.get_chain(), "BTC");
	assert_eq!(btc.get_chain_id().as_deref(), Some("bitcoin"));
	assert_eq!(btc.get_display_name().as_deref(), None);
	assert_eq!(btc.get_providers().as_ref(), None);
	assert_eq!(btc.get_default_decimals(), &None);
	assert!(btc.get_gas_asset().is_none());
	assert!(!btc.get_evm());
}

#[test]
fn gas_history_deserialize() {
	let history: GasHistory = serde_json::from_str(&fixture("gas_history.json")).expect("gas_history.json");

	assert_eq!(history.get_chain_id(), "43114");
	assert_eq!(history.get_unit_name(), "wei");
	assert_eq!(history.get_last_timestamp(), &dec("1688396693338"));
	assert_eq!(history.get_history().len(), 2);
	assert_eq!(history.get_history()[0].get_value(), &dec("25"));
	assert_eq!(history.get_history()[1].get_value(), &dec("25.5"));

	// Full float precision, not the ~15 significant digits `Decimal::from_f64` would keep.
	assert_eq!(history.get_average_24h(), &dec("25.01171800476548"));
	assert_eq!(history.get_average_7d(), &dec("25.46623886101168"));
}

#[test]
fn providers_deserialize() {
	let providers: Vec<Provider> = serde_json::from_str(&fixture("providers.json")).expect("providers.json");

	assert_eq!(providers.len(), 2);
	assert_eq!(providers[0].get_provider(), "Woofi");
	assert_eq!(providers[0].get_nb_tokens(), &dec("4"));
	assert_eq!(providers[0].get_version().get_major(), &dec("1"));

	// Numeric strings and a null patch version: the tolerant-decimal contract again, on a type
	// that is not a gas price.
	assert_eq!(providers[1].get_nb_tokens(), &dec("129"));
	assert_eq!(providers[1].get_version().get_major(), &dec("2"));
	assert_eq!(providers[1].get_version().get_patch(), &Decimal::ZERO);
}

#[test]
fn supported_providers_deserialize() {
	let providers: SupportedProviders = serde_json::from_str(&fixture("supported_providers.json")).expect("supported_providers.json");

	assert_eq!(providers.get_providers().len(), 6);
	assert!(providers.get_providers().iter().any(|p| p == "THORCHAIN"));
}

#[test]
fn exchange_rate_deserialize() {
	let rate: ExchangeRate = serde_json::from_str(&fixture("exchange_rate.json")).expect("exchange_rate.json");

	// The price arrives as a quoted string and must survive as an exact decimal.
	assert_eq!(rate.get_price(), &dec("1040560352527"));
}
