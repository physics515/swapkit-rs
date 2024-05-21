use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
		"assetAmount": "0.04505466",
		"baseAmount": "4505466",
		"decimal": 8,
		"asset": {
				"chain": "BTC",
				"symbol": "BTC",
				"ticker": "BTC",
				"type": "Native",
				"network": "Bitcoin",
				"name": "BTC",
				"decimal": 8,
				"isSynth": false
		},
		"amount": {
				"assetAmount": "0.04505466",
				"baseAmount": "4505466",
				"decimal": 8
		}
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteExpectedAmountOut {
	#[serde(rename = "assetAmount", with = "rust_decimal::serde::str")]
	asset_amount: Decimal,

	#[serde(rename = "baseAmount", with = "rust_decimal::serde::str")]
	base_amount: Decimal,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize")]
	decimal: Decimal,
}

impl QuoteExpectedAmountOut {
	#[must_use]
	pub const fn get_asset_amount(&self) -> Decimal {
		self.asset_amount
	}

	#[must_use]
	pub const fn get_base_amount(&self) -> Decimal {
		self.base_amount
	}

	#[must_use]
	pub const fn get_decimal(&self) -> Decimal {
		self.decimal
	}
}
