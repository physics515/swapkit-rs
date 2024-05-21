use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
		"chain": "AVAX",
		"symbol": "AVAX",
		"identifier": "AVAX.AVAX",
		"decimals": 18
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasAsset {
	chain: String,
	symbol: String,
	identifier: String,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize")]
	decimals: Decimal,
}

impl GasAsset {
	#[must_use]
	pub const fn get_chain(&self) -> &String {
		&self.chain
	}

	#[must_use]
	pub const fn get_symbol(&self) -> &String {
		&self.symbol
	}

	#[must_use]
	pub const fn get_identifier(&self) -> &String {
		&self.identifier
	}

	#[must_use]
	pub const fn get_decimals(&self) -> &Decimal {
		&self.decimals
	}
}
