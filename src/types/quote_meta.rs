use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::QuoteThornodeMeta;
use crate::QuoteWarning;

/*
{
		"hasStreamingSwap": false,
		"slippagePercentage": 1,
		"sellChain": "ETH",
		"sellChainGasRate": "10",
		"buyChain": "BTC",
		"buyChainGasRate": "24",
		"priceProtectionRequired": false,
		"priceProtectionDetected": true,
		"quoteMode": "TC-TC",
		"thornodeMeta": QuoteThornodeMeta,
		"recommendedSlippage": 1,
		"warnings": Vec<QuoteWarning>
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteMeta {
	#[serde(rename = "hasStreamingSwap")]
	has_streaming_swap: bool,

	#[serde(rename = "slippagePercentage", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	slippage_percentage: Decimal,

	#[serde(rename = "sellChain")]
	sell_chain: String,

	#[serde(rename = "sellChainGasRate", with = "rust_decimal::serde::str_option")]
	sell_chain_gas_rate: Option<Decimal>,

	#[serde(rename = "buyChain")]
	buy_chain: String,

	#[serde(rename = "buyChainGasRate", with = "rust_decimal::serde::str")]
	buy_chain_gas_rate: Decimal,

	#[serde(rename = "priceProtectionRequired")]
	price_protection_required: bool,

	#[serde(rename = "priceProtectionDetected")]
	price_protection_detected: bool,

	#[serde(rename = "quoteMode")]
	quote_mode: String,

	#[serde(rename = "thornodeMeta")]
	thornode_meta: QuoteThornodeMeta,

	#[serde(rename = "recommendedSlippage", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	recommended_slippage: Decimal,

	warnings: Vec<QuoteWarning>,
}

impl QuoteMeta {
	#[must_use]
	pub const fn get_has_streaming_swap(&self) -> bool {
		self.has_streaming_swap
	}

	#[must_use]
	pub const fn get_slippage_percentage(&self) -> Decimal {
		self.slippage_percentage
	}

	#[must_use]
	pub const fn get_sell_chain(&self) -> &String {
		&self.sell_chain
	}

	#[must_use]
	pub const fn get_sell_chain_gas_rate(&self) -> Option<Decimal> {
		self.sell_chain_gas_rate
	}

	#[must_use]
	pub const fn get_buy_chain(&self) -> &String {
		&self.buy_chain
	}

	#[must_use]
	pub const fn get_buy_chain_gas_rate(&self) -> Decimal {
		self.buy_chain_gas_rate
	}

	#[must_use]
	pub const fn get_price_protection_required(&self) -> bool {
		self.price_protection_required
	}

	#[must_use]
	pub const fn get_price_protection_detected(&self) -> bool {
		self.price_protection_detected
	}

	#[must_use]
	pub const fn get_quote_mode(&self) -> &String {
		&self.quote_mode
	}

	#[must_use]
	pub const fn get_thornode_meta(&self) -> &QuoteThornodeMeta {
		&self.thornode_meta
	}

	#[must_use]
	pub const fn get_recommended_slippage(&self) -> Decimal {
		self.recommended_slippage
	}

	#[must_use]
	pub const fn get_warnings(&self) -> &Vec<QuoteWarning> {
		&self.warnings
	}
}
