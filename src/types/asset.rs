use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
	"chain": "THOR",
	"symbol": "RUNE",
	"ticker": "RUNE",
	"type": "Native",
	"network": "THORChain",
	"name": "RUNE",
	"decimal": 8,
	"isSynth": false
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
	chain: String,
	symbol: String,
	ticker: String,

	#[serde(rename = "type")]
	asset_type: String,

	network: String,
	name: String,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize")]
	decimal: Decimal,

	#[serde(rename = "isSynth")]
	is_synth: bool,
}

impl Asset {
	#[must_use]
	pub const fn get_chain(&self) -> &str {
		&self.chain
	}

	#[must_use]
	pub const fn get_symbol(&self) -> &str {
		&self.symbol
	}

	#[must_use]
	pub const fn get_ticker(&self) -> &str {
		&self.ticker
	}

	#[must_use]
	pub const fn get_asset_type(&self) -> &str {
		&self.asset_type
	}

	#[must_use]
	pub const fn get_network(&self) -> &str {
		&self.network
	}

	#[must_use]
	pub const fn get_name(&self) -> &str {
		&self.name
	}

	#[must_use]
	pub const fn get_decimal(&self) -> Decimal {
		self.decimal
	}

	#[must_use]
	pub const fn is_synth(&self) -> bool {
		self.is_synth
	}

	#[must_use]
	pub const fn get_is_synth(&self) -> bool {
		self.is_synth
	}
}
