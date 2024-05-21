use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::QuoteExpectedAmountOut;
use crate::QuoteThornodeMetaFees;

/*
{
		"expectedAmountOut": QuoteExpectedAmountOut,
		"expectedAmountOutStreaming": QuoteExpectedAmountOut,
		"expiry": 1715879102,
		"fees": QuoteThornodeMetaFees,
		"inboundAddress": "0xcb4c6ae4f20477f96342467df478ae83512a7bd5",
		"inboundConfirmationBlocks": 2,
		"inboundConfirmationSeconds": 24,
		"maxStreamingQuantity": 1,
		"memo": "=:BTC.BTC:1Lbcfr7sAHTD9CgdQo3HTMTkV8LK4ZnX71:0/1/1:t:30",
		"notes": "Base Asset: Send the inbound_address the asset with the memo encoded in hex in the data field. Tokens: First approve router to spend tokens from user: asset.approve(router, amount). Then call router.depositWithExpiry(inbound_address, asset, amount, memo, expiry). Asset is the token contract address. Amount should be in native asset decimals (eg 1e18 for most tokens). Do not send to or from contract addresses.",
		"outboundDelayBlocks": 9,
		"outboundDelaySeconds": 54,
		"recommendedMinAmountIn": "2872899",
		"router": "0xD37BbE5744D730a1d98d8DC97c42F0Ca46aD7146",
		"slippageBps": 1,
		"streamingSlippageBps": 1,
		"streamingSwapBlocks": 0,
		"streamingSwapSeconds": 0,
		"totalSwapSeconds": 78,
		"warning": "Do not cache this response. Do not send funds after the expiry."
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteThornodeMeta {
	#[serde(rename = "expectedAmountOut")]
	expected_amount_out: QuoteExpectedAmountOut,

	#[serde(rename = "expectedAmountOutStreaming")]
	expected_amount_out_streaming: QuoteExpectedAmountOut,

	#[serde(rename = "expiry", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	expiry: Decimal,

	fees: QuoteThornodeMetaFees,

	#[serde(rename = "inboundAddress")]
	inbound_address: Option<String>,

	#[serde(rename = "inboundConfirmationBlocks", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	inbound_confirmation_blocks: Decimal,

	#[serde(rename = "inboundConfirmationSeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	inbound_confirmation_seconds: Decimal,

	#[serde(rename = "maxStreamingQuantity", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	max_streaming_quantity: Decimal,

	memo: String,

	notes: String,

	#[serde(rename = "outboundDelayBlocks", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	outbound_delay_blocks: Decimal,

	#[serde(rename = "outboundDelaySeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	outbound_delay_seconds: Decimal,

	#[serde(rename = "recommendedMinAmountIn")]
	recommended_min_amount_in: String,

	router: Option<String>,

	#[serde(rename = "slippageBps", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	slippage_bps: Decimal,

	#[serde(rename = "streamingSlippageBps", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	streaming_slippage_bps: Decimal,

	#[serde(rename = "streamingSwapBlocks", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	streaming_swap_blocks: Decimal,

	#[serde(rename = "streamingSwapSeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	streaming_swap_seconds: Decimal,

	#[serde(rename = "totalSwapSeconds", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize")]
	total_swap_seconds: Option<Decimal>,

	warning: String,
}

impl QuoteThornodeMeta {
	#[must_use]
	pub const fn get_expected_amount_out(&self) -> &QuoteExpectedAmountOut {
		&self.expected_amount_out
	}

	#[must_use]
	pub const fn get_expected_amount_out_streaming(&self) -> &QuoteExpectedAmountOut {
		&self.expected_amount_out_streaming
	}

	#[must_use]
	pub const fn get_expiry(&self) -> Decimal {
		self.expiry
	}

	#[must_use]
	pub const fn get_fees(&self) -> &QuoteThornodeMetaFees {
		&self.fees
	}

	#[must_use]
	pub const fn get_inbound_address(&self) -> &Option<String> {
		&self.inbound_address
	}

	#[must_use]
	pub const fn get_inbound_confirmation_blocks(&self) -> Decimal {
		self.inbound_confirmation_blocks
	}

	#[must_use]
	pub const fn get_inbound_confirmation_seconds(&self) -> Decimal {
		self.inbound_confirmation_seconds
	}

	#[must_use]
	pub const fn get_max_streaming_quantity(&self) -> Decimal {
		self.max_streaming_quantity
	}

	#[must_use]
	pub const fn get_memo(&self) -> &String {
		&self.memo
	}

	#[must_use]
	pub const fn get_notes(&self) -> &String {
		&self.notes
	}

	#[must_use]
	pub const fn get_outbound_delay_blocks(&self) -> Decimal {
		self.outbound_delay_blocks
	}

	#[must_use]
	pub const fn get_outbound_delay_seconds(&self) -> Decimal {
		self.outbound_delay_seconds
	}

	#[must_use]
	pub const fn get_recommended_min_amount_in(&self) -> &String {
		&self.recommended_min_amount_in
	}

	#[must_use]
	pub const fn get_router(&self) -> &Option<String> {
		&self.router
	}

	#[must_use]
	pub const fn get_slippage_bps(&self) -> Decimal {
		self.slippage_bps
	}

	#[must_use]
	pub const fn get_streaming_slippage_bps(&self) -> Decimal {
		self.streaming_slippage_bps
	}

	#[must_use]
	pub const fn get_streaming_swap_blocks(&self) -> Decimal {
		self.streaming_swap_blocks
	}

	#[must_use]
	pub const fn get_streaming_swap_seconds(&self) -> Decimal {
		self.streaming_swap_seconds
	}

	#[must_use]
	pub const fn get_total_swap_seconds(&self) -> Option<Decimal> {
		self.total_swap_seconds
	}

	#[must_use]
	pub const fn get_warning(&self) -> &String {
		&self.warning
	}
}
