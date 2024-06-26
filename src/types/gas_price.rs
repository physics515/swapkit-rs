use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::deserialize_rust_decimal_from_anything;


/*
{
		"asset": "THOR.RUNE",
		"units": "tor",
		"gas": 2000000,
		"chainId": "thorchain-mainnet-v1",
		"gasAsset": 0.02
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasPrice {
	asset: String,
	units: String,

	#[serde( serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "deserialize_rust_decimal_from_anything")]
	gas: Decimal,

	#[serde(rename = "chainId")]
	chain_id: String,

	#[serde(rename = "gasAsset", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "deserialize_rust_decimal_from_anything")]
	gas_asset: Decimal,
}

impl GasPrice {
	#[must_use]
	pub const fn get_asset(&self) -> &String {
		&self.asset
	}

	#[must_use]
	pub const fn get_units(&self) -> &String {
		&self.units
	}

	#[must_use]
	pub const fn get_gas(&self) -> &Decimal {
		&self.gas
	}

	#[must_use]
	pub const fn get_chain_id(&self) -> &String {
		&self.chain_id
	}

	#[must_use]
	pub const fn get_gas_asset(&self) -> &Decimal {
		&self.gas_asset
	}
}
