use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::QuoteWarning;

/*
{
		"from": "0xb794f5ea0ba39494ce839613fffba74279579268",
		"to": "0xD37BbE5744D730a1d98d8DC97c42F0Ca46aD7146",
		"value": "0xde0b6b3a7640000",
		"data": "0x44bc937b000000000000000000000000cb4c6ae4f20477f96342467df478ae83512a7bd500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000006646474b00000000000000000000000000000000000000000000000000000000000000333d3a623a314c6263667237734148544439436764516f3348544d546b56384c4b345a6e5837313a343436303835373a743a333000000000000000000000000000",
		"gas": "0xaec8",
		"gasPrice": 7
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteTransaction {
	from: String,
	to: String,
	value: String,
	data: String,
	gas: String,

	#[serde(rename = "gasPrice", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	gas_price: Decimal,
}

impl QuoteTransaction {
	#[must_use]
	pub const fn get_from(&self) -> &String {
		&self.from
	}

	#[must_use]
	pub const fn get_to(&self) -> &String {
		&self.to
	}

	#[must_use]
	pub const fn get_value(&self) -> &String {
		&self.value
	}

	#[must_use]
	pub const fn get_data(&self) -> &String {
		&self.data
	}

	#[must_use]
	pub const fn get_gas(&self) -> &String {
		&self.gas
	}

	#[must_use]
	pub const fn get_gas_price(&self) -> Decimal {
		self.gas_price
	}
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuoteTransactionEnum {
	QuoteTransaction(QuoteTransaction),
	QuoteWarning(Vec<QuoteWarning>),
}
