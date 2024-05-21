use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{Asset, AssetPrice};

/*
{
	"assetAmount": "5232866.89214283",
	"baseAmount": "523286689214283",
	"decimal": 8,
	"asset": {
		"chain": "THOR",
		"symbol": "RUNE",
		"ticker": "RUNE",
		"type": "Native",
		"network": "THORChain",
		"name": "RUNE",
		"decimal": 8,
		"isSynth": false
	},
	"amount": {
		"assetAmount": "5232866.89214283",
		"baseAmount": "523286689214283",
		"decimal": 8
	}
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Depth {
	#[serde(rename = "assetAmount", with = "rust_decimal::serde::str")]
	asset_amount: Decimal,

	#[serde(rename = "baseAmount", with = "rust_decimal::serde::str")]
	base_amount: Decimal,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize")]
	decimal: Decimal,
	asset: Asset,
	amount: AssetPrice,
}

impl Depth {
	#[must_use]
	pub const fn get_asset_amount(&self) -> &Decimal {
		&self.asset_amount
	}

	#[must_use]
	pub const fn get_base_amount(&self) -> &Decimal {
		&self.base_amount
	}

	#[must_use]
	pub const fn get_decimal(&self) -> &Decimal {
		&self.decimal
	}

	#[must_use]
	pub const fn get_asset(&self) -> &Asset {
		&self.asset
	}

	#[must_use]
	pub const fn get_amount(&self) -> &AssetPrice {
		&self.amount
	}
}
