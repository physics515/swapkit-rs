use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
		"from": "ETH.ETH",
		"to": "BTC.BTC",
		"parts": [
				{
						"provider": "THORCHAIN",
						"percentage": 100
				}
		]
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteSwap {
	from: String,
	to: String,
	parts: Vec<QuoteSwapPart>,
}

impl QuoteSwap {
	#[must_use]
	pub const fn get_from(&self) -> &String {
		&self.from
	}

	#[must_use]
	pub const fn get_to(&self) -> &String {
		&self.to
	}

	#[must_use]
	pub const fn get_parts(&self) -> &Vec<QuoteSwapPart> {
		&self.parts
	}
}

/*
{
		"provider": "THORCHAIN",
		"percentage": 100
}
*/

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteSwapPart {
	provider: String,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	percentage: Decimal,
}

impl QuoteSwapPart {
	#[must_use]
	pub const fn get_provider(&self) -> &String {
		&self.provider
	}

	#[must_use]
	pub const fn get_percentage(&self) -> Decimal {
		self.percentage
	}
}
