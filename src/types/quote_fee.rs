use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
		"type": "inbound",
		"asset": "ETH.ETH",
		"networkFee": 0.000313208,
		"networkFeeUSD": 0.9248343182400001,
		"affiliateFee": 0,
		"affiliateFeeUSD": 0,
		"totalFee": 0.000313208,
		"totalFeeUSD": 0.9248343182400001,
		"isOutOfPocket": true
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteFee {
	#[serde(rename = "type")]
	fee_type: String,

	asset: String,

	#[serde(rename = "networkFee", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	network_fee: Decimal,

	#[serde(rename = "networkFeeUSD", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	network_fee_usd: Decimal,

	#[serde(rename = "affiliateFee", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	affiliate_fee: Decimal,

	#[serde(rename = "affiliateFeeUSD", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	affiliate_fee_usd: Decimal,

	#[serde(rename = "totalFee", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	total_fee: Decimal,

	#[serde(rename = "totalFeeUSD", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	total_fee_usd: Decimal,

	#[serde(rename = "isOutOfPocket")]
	is_out_of_pocket: bool,
}

impl QuoteFee {
	#[must_use]
	pub const fn get_fee_type(&self) -> &String {
		&self.fee_type
	}

	#[must_use]
	pub const fn get_asset(&self) -> &String {
		&self.asset
	}

	#[must_use]
	pub const fn get_network_fee(&self) -> Decimal {
		self.network_fee
	}

	#[must_use]
	pub const fn get_network_fee_usd(&self) -> Decimal {
		self.network_fee_usd
	}

	#[must_use]
	pub const fn get_affiliate_fee(&self) -> Decimal {
		self.affiliate_fee
	}

	#[must_use]
	pub const fn get_affiliate_fee_usd(&self) -> Decimal {
		self.affiliate_fee_usd
	}

	#[must_use]
	pub const fn get_total_fee(&self) -> Decimal {
		self.total_fee
	}

	#[must_use]
	pub const fn get_total_fee_usd(&self) -> Decimal {
		self.total_fee_usd
	}

	#[must_use]
	pub const fn get_is_out_of_pocket(&self) -> bool {
		self.is_out_of_pocket
	}
}
