use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
		"_address": "bc1qhz06s94etnd2q47qwkxuq4qv3y7exngu5m5vkp",
		"chain": "BTC",
		"chainLpActionsPaused": false,
		"chainTradingPaused": false,
		"dustThreshold": "10000",
		"gasRate": "21",
		"gasRateUnits": "satsperbyte",
		"globalTradingPaused": false,
		"halted": false,
		"outboundFee": "6476",
		"outboundTxSize": "1000",
		"pubKey": "thorpub1addwnpepqvgm96hrjudgklqslm855dkfy4r005ve0rc33xtvccemr4pyd8s5vegmrjt",
		"verified": true,
		"verifiedAt": 1715804720329,
		"accessedAt": -1
}
*/

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundAddress {
	#[serde(rename = "_address")]
	address: String,

	chain: String,

	#[serde(rename = "chainLpActionsPaused")]
	chain_lp_actions_paused: bool,

	#[serde(rename = "chainTradingPaused")]
	chain_trading_paused: bool,

	#[serde(rename = "dustThreshold", with = "rust_decimal::serde::str")]
	dust_threshold: Decimal,

	#[serde(rename = "gasRate", with = "rust_decimal::serde::str")]
	gas_rate: Decimal,

	#[serde(rename = "gasRateUnits")]
	gas_rate_units: String,

	#[serde(rename = "globalTradingPaused")]
	global_trading_paused: bool,

	halted: bool,

	#[serde(rename = "outboundFee", with = "rust_decimal::serde::str")]
	outbound_fee: Decimal,

	#[serde(rename = "outboundTxSize", with = "rust_decimal::serde::str")]
	outbound_tx_size: Decimal,

	#[serde(rename = "pubKey")]
	pub_key: String,

	verified: bool,

	#[serde(rename = "verifiedAt", serialize_with = "rust_decimal::serde::str::serialize")]
	verified_at: Decimal,

	#[serde(rename = "accessedAt", serialize_with = "rust_decimal::serde::str::serialize")]
	accessed_at: Decimal,
}

impl InboundAddress {
	#[must_use]
	pub const fn get_address(&self) -> &String {
		&self.address
	}

	#[must_use]
	pub const fn get_chain(&self) -> &String {
		&self.chain
	}

	#[must_use]
	pub const fn get_chain_lp_actions_paused(&self) -> &bool {
		&self.chain_lp_actions_paused
	}

	#[must_use]
	pub const fn get_chain_trading_paused(&self) -> &bool {
		&self.chain_trading_paused
	}

	#[must_use]
	pub const fn get_dust_threshold(&self) -> &Decimal {
		&self.dust_threshold
	}

	#[must_use]
	pub const fn get_gas_rate(&self) -> &Decimal {
		&self.gas_rate
	}

	#[must_use]
	pub const fn get_gas_rate_units(&self) -> &String {
		&self.gas_rate_units
	}

	#[must_use]
	pub const fn get_global_trading_paused(&self) -> &bool {
		&self.global_trading_paused
	}

	#[must_use]
	pub const fn get_halted(&self) -> &bool {
		&self.halted
	}

	#[must_use]
	pub const fn get_outbound_fee(&self) -> &Decimal {
		&self.outbound_fee
	}

	#[must_use]
	pub const fn get_outbound_tx_size(&self) -> &Decimal {
		&self.outbound_tx_size
	}

	#[must_use]
	pub const fn get_pub_key(&self) -> &String {
		&self.pub_key
	}

	#[must_use]
	pub const fn get_verified(&self) -> &bool {
		&self.verified
	}

	#[must_use]
	pub const fn get_verified_at(&self) -> &Decimal {
		&self.verified_at
	}

	#[must_use]
	pub const fn get_accessed_at(&self) -> &Decimal {
		&self.accessed_at
	}
}
