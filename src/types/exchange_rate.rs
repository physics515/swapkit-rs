use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
		"price": "1040560352527"
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
	#[serde(with = "rust_decimal::serde::str")]
	price: Decimal,
}

impl ExchangeRate {
	#[must_use]
	pub const fn get_price(&self) -> &Decimal {
		&self.price
	}
}
