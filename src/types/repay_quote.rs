use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{QuoteFee, QuoteTimeEstimates, StreamingSwap};

/*
{
		"inboundAddress": "bc1qpqkqxgln9qwahw76nyp3v9fqyc7zy252yhv2kc",
		"inboundConfirmationBlocks": 1,
		"inboundConfirmationSeconds": 600,
		"outboundDelayBlocks": 0,
		"outboundDelaySeconds": 0,
		"fees": Hashmap<String, Vec<QuoteFee>>
		"expiry": 1715977820,
		"warning": "Do not cache this response. Do not send funds after the expiry.",
		"notes": "First output should be to inbound_address, second output should be change back to self, third output should be OP_RETURN, limited to 80 bytes. Do not send below the dust threshold. Do not use exotic spend scripts, locks or address formats (P2WSH with Bech32 address format preferred).",
		"dustThreshold": "10000",
		"recommendedMinAmountIn": "0.00031104",
		"memo": "$-:BTC.BTC:bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz:1",
		"expectedAmountOut": "0.00000000",
		"expectedAmountIn": "0.00013192",
		"expectedCollateralWithdrawn": "0.00000000",
		"expectedDebtRepaid": "8.82417067",
		"streamingSwapBlocks": 2,
		"streamingSwapSeconds": 12,
		"totalRepaySeconds": 612,
		"collateralCurrent": "0.06489696",
		"repayAssetAmount": "0.00019534",
		"repayAssetAmountUSD": "12.25657330",
		"timeEstimates": QuoteTimeEstimates,
		"streamingSwap": StreamingSwap
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepayQuote {
	#[serde(rename = "inboundAddress")]
	inbound_address: String,

	#[serde(rename = "inboundConfirmationBlocks", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	inbound_confirmation_blocks: Decimal,

	#[serde(rename = "inboundConfirmationSeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	inbound_confirmation_seconds: Decimal,

	#[serde(rename = "outboundDelayBlocks", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	outbound_delay_blocks: Decimal,

	#[serde(rename = "outboundDelaySeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	outbound_delay_seconds: Decimal,

	fees: HashMap<String, Vec<QuoteFee>>,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	expiry: Decimal,

	warning: String,

	notes: String,

	#[serde(rename = "dustThreshold")]
	dust_threshold: String,

	#[serde(rename = "recommendedMinAmountIn")]
	recommended_min_amount_in: String,

	memo: String,

	#[serde(rename = "expectedAmountOut")]
	expected_amount_out: String,

	#[serde(rename = "expectedAmountIn")]
	expected_amount_in: String,

	#[serde(rename = "expectedCollateralWithdrawn")]
	expected_collateral_withdrawn: String,

	#[serde(rename = "expectedDebtRepaid")]
	expected_debt_repaid: String,

	#[serde(rename = "streamingSwapBlocks", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	streaming_swap_blocks: Decimal,

	#[serde(rename = "streamingSwapSeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	streaming_swap_seconds: Decimal,

	#[serde(rename = "totalRepaySeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	total_repay_seconds: Decimal,

	#[serde(rename = "collateralCurrent")]
	collateral_current: String,

	#[serde(rename = "repayAssetAmount")]
	repay_asset_amount: String,

	#[serde(rename = "repayAssetAmountUSD")]
	repay_asset_amount_usd: String,

	#[serde(rename = "timeEstimates")]
	time_estimates: QuoteTimeEstimates,

	#[serde(rename = "streamingSwap")]
	streaming_swap: StreamingSwap,
}

impl RepayQuote {
	#[must_use]
	pub fn get_inbound_address(&self) -> &str {
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
	pub const fn get_outbound_delay_blocks(&self) -> Decimal {
		self.outbound_delay_blocks
	}

	#[must_use]
	pub const fn get_outbound_delay_seconds(&self) -> Decimal {
		self.outbound_delay_seconds
	}

	#[must_use]
	pub const fn get_fees(&self) -> &HashMap<String, Vec<QuoteFee>> {
		&self.fees
	}

	#[must_use]
	pub const fn get_expiry(&self) -> Decimal {
		self.expiry
	}

	#[must_use]
	pub fn get_warning(&self) -> &str {
		&self.warning
	}

	#[must_use]
	pub fn get_notes(&self) -> &str {
		&self.notes
	}

	#[must_use]
	pub fn get_dust_threshold(&self) -> &str {
		&self.dust_threshold
	}

	#[must_use]
	pub fn get_recommended_min_amount_in(&self) -> &str {
		&self.recommended_min_amount_in
	}

	#[must_use]
	pub fn get_memo(&self) -> &str {
		&self.memo
	}

	#[must_use]
	pub fn get_expected_amount_out(&self) -> &str {
		&self.expected_amount_out
	}

	#[must_use]
	pub fn get_expected_amount_in(&self) -> &str {
		&self.expected_amount_in
	}

	#[must_use]
	pub fn get_expected_collateral_withdrawn(&self) -> &str {
		&self.expected_collateral_withdrawn
	}

	#[must_use]
	pub fn get_expected_debt_repaid(&self) -> &str {
		&self.expected_debt_repaid
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
	pub const fn get_total_repay_seconds(&self) -> Decimal {
		self.total_repay_seconds
	}

	#[must_use]
	pub fn get_collateral_current(&self) -> &str {
		&self.collateral_current
	}

	#[must_use]
	pub fn get_repay_asset_amount(&self) -> &str {
		&self.repay_asset_amount
	}

	#[must_use]
	pub fn get_repay_asset_amount_usd(&self) -> &str {
		&self.repay_asset_amount_usd
	}

	#[must_use]
	pub const fn get_time_estimates(&self) -> &QuoteTimeEstimates {
		&self.time_estimates
	}

	#[must_use]
	pub const fn get_streaming_swap(&self) -> &StreamingSwap {
		&self.streaming_swap
	}
}
