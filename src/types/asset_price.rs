use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
	"assetAmount": "5232866.89214283",
	"baseAmount": "523286689214283",
	"decimal": 8
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetPrice {
	#[serde(rename = "assetAmount", with = "rust_decimal::serde::str")]
	asset_amount: Decimal,

	#[serde(rename = "baseAmount", with = "rust_decimal::serde::str")]
	base_amount: Decimal,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize")]
	decimal: Decimal,
}

impl AssetPrice {
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
