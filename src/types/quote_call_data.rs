use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
		"depositWithExpiry": "1",
		"vault": "0xcb4c6ae4f20477f96342467df478ae83512a7bd5",
		"asset": "ETH.ETH",
		"amount": "1000000000000000000",
		"memo": "=:b:1Lbcfr7sAHTD9CgdQo3HTMTkV8LK4ZnX71:4460857:t:30",
		"memoStreamingSwap": "",
		"expiration": "1715881803",
		"fromAsset": "ETH.ETH",
		"amountIn": "1000000000000000000"
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteCallData {
	#[serde(rename = "depositWithExpiry", with = "rust_decimal::serde::str")]
	deposit_with_expiry: Decimal,

	vault: Option<String>,

	asset: String,

	#[serde(with = "rust_decimal::serde::str")]
	amount: Decimal,

	memo: String,

	#[serde(rename = "memoStreamingSwap")]
	memo_streaming_swap: String,

	#[serde(with = "rust_decimal::serde::str")]
	expiration: Decimal,

	#[serde(rename = "fromAsset")]
	from_asset: String,

	#[serde(rename = "amountIn", with = "rust_decimal::serde::str")]
	amount_in: Decimal,
}

impl QuoteCallData {
	#[must_use]
	pub const fn get_deposit_with_expiry(&self) -> Decimal {
		self.deposit_with_expiry
	}

	#[must_use]
	pub const fn get_vault(&self) -> &Option<String> {
		&self.vault
	}

	#[must_use]
	pub const fn get_asset(&self) -> &String {
		&self.asset
	}

	#[must_use]
	pub const fn get_amount(&self) -> Decimal {
		self.amount
	}

	#[must_use]
	pub const fn get_memo(&self) -> &String {
		&self.memo
	}

	#[must_use]
	pub const fn get_memo_streaming_swap(&self) -> &String {
		&self.memo_streaming_swap
	}

	#[must_use]
	pub const fn get_expiration(&self) -> Decimal {
		self.expiration
	}

	#[must_use]
	pub const fn get_from_asset(&self) -> &String {
		&self.from_asset
	}

	#[must_use]
	pub const fn get_amount_in(&self) -> Decimal {
		self.amount_in
	}
}
