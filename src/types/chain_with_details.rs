use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::deserialize_string_option_from_number;
use crate::GasAsset;

/*
{
		"chain": "AVAX",
		"chainId": 43114,
		"displayName": "Avalanche",
		"symbol": "AVAX",
		"logo": "https://static.thorswap.net/token-list/images/avax.avax.png",
		"providers": [
				"THORCHAIN",
				"PANGOLIN"
		],
		"status": "active",
		"category": "evm",
		"evm": true,
		"mainnet": true,
		"defaultDecimals": 18,
		"averageBlockTime": 3000,
		"confirmationsRequired": 1,
		"gasRate": null,
		"gasAsset": {
				"chain": "AVAX",
				"symbol": "AVAX",
				"identifier": "AVAX.AVAX",
				"decimals": 18
		}
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainWithDetails {
	chain: String,

	#[serde(rename = "chainId", deserialize_with = "deserialize_string_option_from_number", default)]
	chain_id: Option<String>,

	#[serde(rename = "displayName", default)]
	display_name: Option<String>,

	#[serde(default)]
	symbol: Option<String>,

	#[serde(default)]
	logo: Option<String>,

	#[serde(default)]
	providers: Option<Vec<String>>,

	status: String,

	#[serde(default)]
	category: Option<String>,

	evm: bool,
	mainnet: bool,

	#[serde(rename = "defaultDecimals", deserialize_with = "rust_decimal::serde::float_option::deserialize", serialize_with = "rust_decimal::serde::str_option::serialize", default)]
	default_decimals: Option<Decimal>,

	#[serde(rename = "averageBlockTime", serialize_with = "rust_decimal::serde::str::serialize")]
	average_block_time: Decimal,

	#[serde(rename = "confirmationsRequired", serialize_with = "rust_decimal::serde::str_option::serialize", default)]
	confirmations_required: Option<Decimal>,

	#[serde(rename = "gasRate", serialize_with = "rust_decimal::serde::str_option::serialize")]
	gas_rate: Option<Decimal>,

	#[serde(rename = "gasAsset", default)]
	gas_asset: Option<GasAsset>,
}

impl ChainWithDetails {
	#[must_use]
	pub const fn get_chain(&self) -> &String {
		&self.chain
	}

	#[must_use]
	pub const fn get_chain_id(&self) -> &Option<String> {
		&self.chain_id
	}

	#[must_use]
	pub const fn get_display_name(&self) -> &Option<String> {
		&self.display_name
	}

	#[must_use]
	pub const fn get_symbol(&self) -> &Option<String> {
		&self.symbol
	}

	#[must_use]
	pub const fn get_logo(&self) -> &Option<String> {
		&self.logo
	}

	#[must_use]
	pub const fn get_providers(&self) -> &Option<Vec<String>> {
		&self.providers
	}

	#[must_use]
	pub const fn get_status(&self) -> &String {
		&self.status
	}

	#[must_use]
	pub const fn get_category(&self) -> &Option<String> {
		&self.category
	}

	#[must_use]
	pub const fn get_evm(&self) -> bool {
		self.evm
	}

	#[must_use]
	pub const fn get_mainnet(&self) -> bool {
		self.mainnet
	}

	#[must_use]
	pub const fn get_default_decimals(&self) -> &Option<Decimal> {
		&self.default_decimals
	}

	#[must_use]
	pub const fn get_average_block_time(&self) -> &Decimal {
		&self.average_block_time
	}

	#[must_use]
	pub const fn get_confirmations_required(&self) -> &Option<Decimal> {
		&self.confirmations_required
	}

	#[must_use]
	pub const fn get_gas_rate(&self) -> Option<&Decimal> {
		self.gas_rate.as_ref()
	}

	#[must_use]
	pub const fn get_gas_asset(&self) -> &Option<GasAsset> {
		&self.gas_asset
	}
}
