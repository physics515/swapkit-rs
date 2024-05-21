use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::deserialize_string_option_from_number;

/*
{
		"name": "JOE",
		"ticker": "JOE",
		"address": "0x6e84a6216eA6dACC71eE8E6b0a5B7322EEbC0fDd",
		"protocol": "AVAX",
		"blockchain": "avalanche",
		"chainId": 43114,
		"apiIdentifier": "LOGO",
		"decimals": 18,
		"fullName": "AVAX.JOE-0X6E84A6216EA6DACC71EE8E6B0A5B7322EEBC0FDD",
		"extraIdName": "",
		"image": "https://raw.githubusercontent.com/traderjoe-xyz/joe-tokenlists/main/logos/0x6e84a6216eA6dACC71eE8E6b0a5B7322EEbC0fDd/logo.png",
		"enabled": true,
		"addressUrl": "https://snowtrace.io/address/",
		"transactionUrl": "https://snowtrace.io/tx/",
		"payinConfirmation": 1,
		"averageBlockTime": 3000,
		"notifications": {}
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyWithDetails {
	name: String,
	ticker: String,
	address: String,
	protocol: String,
	blockchain: String,

	#[serde(rename = "chainId", deserialize_with = "deserialize_string_option_from_number", default)]
	chain_id: Option<String>,

	#[serde(rename = "apiIdentifier", default)]
	api_identifier: String,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize")]
	decimals: Decimal,

	#[serde(rename = "fullName", default)]
	full_name: String,

	#[serde(rename = "extraIdName", default)]
	extra_id_name: String,

	image: String,
	enabled: bool,

	#[serde(rename = "addressUrl", default)]
	address_url: String,

	#[serde(rename = "transactionUrl", default)]
	transaction_url: String,

	#[serde(rename = "payinConfirmation", serialize_with = "rust_decimal::serde::str_option::serialize")]
	payin_confirmation: Option<Decimal>,

	#[serde(rename = "averageBlockTime", serialize_with = "rust_decimal::serde::str_option::serialize")]
	average_block_time: Option<Decimal>,

	notifications: serde_json::Value,
}

impl CurrencyWithDetails {
	#[must_use]
	pub const fn get_name(&self) -> &String {
		&self.name
	}

	#[must_use]
	pub const fn get_ticker(&self) -> &String {
		&self.ticker
	}

	#[must_use]
	pub const fn get_address(&self) -> &String {
		&self.address
	}

	#[must_use]
	pub const fn get_protocol(&self) -> &String {
		&self.protocol
	}

	#[must_use]
	pub const fn get_blockchain(&self) -> &String {
		&self.blockchain
	}

	#[must_use]
	pub const fn get_chain_id(&self) -> &Option<String> {
		&self.chain_id
	}

	#[must_use]
	pub const fn get_api_identifier(&self) -> &String {
		&self.api_identifier
	}

	#[must_use]
	pub const fn get_decimals(&self) -> &Decimal {
		&self.decimals
	}

	#[must_use]
	pub const fn get_full_name(&self) -> &String {
		&self.full_name
	}

	#[must_use]
	pub const fn get_extra_id_name(&self) -> &String {
		&self.extra_id_name
	}

	#[must_use]
	pub const fn get_image(&self) -> &String {
		&self.image
	}

	#[must_use]
	pub const fn get_enabled(&self) -> bool {
		self.enabled
	}

	#[must_use]
	pub const fn get_address_url(&self) -> &String {
		&self.address_url
	}

	#[must_use]
	pub const fn get_transaction_url(&self) -> &String {
		&self.transaction_url
	}

	#[must_use]
	pub const fn get_payin_confirmation(&self) -> &Option<Decimal> {
		&self.payin_confirmation
	}

	#[must_use]
	pub const fn get_average_block_time(&self) -> &Option<Decimal> {
		&self.average_block_time
	}

	#[must_use]
	pub const fn get_notifications(&self) -> &serde_json::Value {
		&self.notifications
	}
}
