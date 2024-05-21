use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{QuoteFee, QuoteTimeEstimates};

/*
{
	   "inboundAddress": "bc1qpqkqxgln9qwahw76nyp3v9fqyc7zy252yhv2kc",
		"inboundConfirmationBlocks": 1,
		"inboundConfirmationSeconds": 600,
		"outboundDelayBlocks": 0,
		"outboundDelaySeconds": 0,
		"fees": HashMap<String, Vec<QuoteFee>>,
		"expectedCollateralDeposited": "0.98938101",
		"expiry": 1715981130,
		"warning": "Do not cache this response. Do not send funds after the expiry.",
		"expectedDebtIssued": "32989.9348",
		"notes": "First output should be to inbound_address, second output should be change back to self, third output should be OP_RETURN, limited to 80 bytes. Do not send below the dust threshold. Do not use exotic spend scripts, locks or address formats (P2WSH with Bech32 address format preferred).",
		"dustThreshold": "10000",
		"recommendedMinAmountIn": "0.00020132",
		"memo": "$-:BTC.BTC:bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz",
		"expectedOutput": "0.49374731",
		"expectedOutputMaxSlippage": "0.49129085572139303483",
		"expectedOutputUSD": "30980.0864883606941174834",
		"expectedOutputMaxSlippageUSD": "30825.9567048365115599440706530708562",
		"expectedAmountOut": "0.00000000",
		"expectedAmountIn": "0.00013191",
		"expectedCollateralWithdrawn": "0.00000000",
		"expectedDebtRepaid": "8.82366454",
		"streamingSwapBlocks": 2,
		"streamingSwapSeconds": 12,
		"totalRepaySeconds": 612,
		"repayAssetAmount": "0.00018987",
		"repayAssetAmountUSD": "11.91335913",
		"timeEstimates": QuoteTimeEstimates
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingSwap {
	#[serde(rename = "inboundAddress")]
	inbound_address: Option<String>,

	#[serde(rename = "inboundConfirmationBlocks", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	inbound_confirmation_blocks: Option<Decimal>,

	#[serde(rename = "inboundConfirmationSeconds", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	inbound_confirmation_seconds: Option<Decimal>,

	#[serde(rename = "outboundDelayBlocks", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	outbound_delay_blocks: Option<Decimal>,

	#[serde(rename = "outboundDelaySeconds", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	outbound_delay_seconds: Option<Decimal>,

	fees: Option<HashMap<String, Vec<QuoteFee>>>,

	#[serde(serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	expiry: Option<Decimal>,

	warning: Option<String>,

	notes: Option<String>,

	#[serde(rename = "dustThreshold", with = "rust_decimal::serde::str_option", default)]
	dust_threshold: Option<Decimal>,

	#[serde(rename = "recommendedMinAmountIn", with = "rust_decimal::serde::str_option", default)]
	recommended_min_amount_in: Option<Decimal>,

	memo: Option<String>,

	#[serde(rename = "expectedAmountOut", with = "rust_decimal::serde::str_option", default)]
	expected_amount_out: Option<Decimal>,

	#[serde(rename = "expectedAmountIn", with = "rust_decimal::serde::str_option", default)]
	expected_amount_in: Option<Decimal>,

	#[serde(rename = "expectedCollateralWithdrawn", with = "rust_decimal::serde::str_option", default)]
	expected_collateral_withdrawn: Option<Decimal>,

	#[serde(rename = "expectedDebtRepaid", with = "rust_decimal::serde::str_option", default)]
	expected_debt_repaid: Option<Decimal>,

	#[serde(rename = "streamingSwapBlocks", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	streaming_swap_blocks: Option<Decimal>,

	#[serde(rename = "streamingSwapSeconds", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	streaming_swap_seconds: Option<Decimal>,

	#[serde(rename = "totalRepaySeconds", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	total_repay_seconds: Option<Decimal>,

	#[serde(rename = "repayAssetAmount", with = "rust_decimal::serde::str_option", default)]
	repay_asset_amount: Option<Decimal>,

	#[serde(rename = "repayAssetAmountUSD", with = "rust_decimal::serde::str_option", default)]
	repay_asset_amount_usd: Option<Decimal>,

	#[serde(rename = "expectedCollateralDeposited", with = "rust_decimal::serde::str_option", default)]
	expected_collateral_deposited: Option<Decimal>,

	#[serde(rename = "expectedDebtIssued", with = "rust_decimal::serde::str_option", default)]
	expected_debt_issued: Option<Decimal>,

	#[serde(rename = "expectedOutput", with = "rust_decimal::serde::str_option", default)]
	expected_output: Option<Decimal>,

	#[serde(rename = "expectedOutputMaxSlippage", with = "rust_decimal::serde::str_option", default)]
	expected_output_max_slippage: Option<Decimal>,

	#[serde(rename = "expectedOutputUSD", with = "rust_decimal::serde::str_option", default)]
	expected_output_usd: Option<Decimal>,

	#[serde(rename = "expectedOutputMaxSlippageUSD", with = "rust_decimal::serde::str_option", default)]
	expected_output_max_slippage_usd: Option<Decimal>,

	#[serde(rename = "timeEstimates")]
	time_estimates: Option<QuoteTimeEstimates>,
}

impl StreamingSwap {
	#[must_use]
	pub const fn get_inbound_address(&self) -> Option<&String> {
		self.inbound_address.as_ref()
	}

	#[must_use]
	pub const fn get_inbound_confirmation_blocks(&self) -> Option<Decimal> {
		self.inbound_confirmation_blocks
	}

	#[must_use]
	pub const fn get_inbound_confirmation_seconds(&self) -> Option<Decimal> {
		self.inbound_confirmation_seconds
	}

	#[must_use]
	pub const fn get_outbound_delay_blocks(&self) -> Option<Decimal> {
		self.outbound_delay_blocks
	}

	#[must_use]
	pub const fn get_outbound_delay_seconds(&self) -> Option<Decimal> {
		self.outbound_delay_seconds
	}

	#[must_use]
	pub const fn get_fees(&self) -> Option<&HashMap<String, Vec<QuoteFee>>> {
		self.fees.as_ref()
	}

	#[must_use]
	pub const fn get_expiry(&self) -> Option<Decimal> {
		self.expiry
	}

	#[must_use]
	pub const fn get_warning(&self) -> Option<&String> {
		self.warning.as_ref()
	}

	#[must_use]
	pub const fn get_notes(&self) -> Option<&String> {
		self.notes.as_ref()
	}

	#[must_use]
	pub const fn get_dust_threshold(&self) -> Option<Decimal> {
		self.dust_threshold
	}

	#[must_use]
	pub const fn get_recommended_min_amount_in(&self) -> Option<Decimal> {
		self.recommended_min_amount_in
	}

	#[must_use]
	pub const fn get_memo(&self) -> Option<&String> {
		self.memo.as_ref()
	}

	#[must_use]
	pub const fn get_expected_amount_out(&self) -> Option<Decimal> {
		self.expected_amount_out
	}

	#[must_use]
	pub const fn get_expected_amount_in(&self) -> Option<Decimal> {
		self.expected_amount_in
	}

	#[must_use]
	pub const fn get_expected_collateral_withdrawn(&self) -> Option<Decimal> {
		self.expected_collateral_withdrawn
	}

	#[must_use]
	pub const fn get_expected_debt_repaid(&self) -> Option<Decimal> {
		self.expected_debt_repaid
	}

	#[must_use]
	pub const fn get_streaming_swap_blocks(&self) -> Option<Decimal> {
		self.streaming_swap_blocks
	}

	#[must_use]
	pub const fn get_streaming_swap_seconds(&self) -> Option<Decimal> {
		self.streaming_swap_seconds
	}

	#[must_use]
	pub const fn get_total_repay_seconds(&self) -> Option<Decimal> {
		self.total_repay_seconds
	}

	#[must_use]
	pub const fn get_repay_asset_amount(&self) -> Option<Decimal> {
		self.repay_asset_amount
	}

	#[must_use]
	pub const fn get_repay_asset_amount_usd(&self) -> Option<Decimal> {
		self.repay_asset_amount_usd
	}

	#[must_use]
	pub const fn get_expected_collateral_deposited(&self) -> Option<Decimal> {
		self.expected_collateral_deposited
	}

	#[must_use]
	pub const fn get_expected_debt_issued(&self) -> Option<Decimal> {
		self.expected_debt_issued
	}

	#[must_use]
	pub const fn get_expected_output(&self) -> Option<Decimal> {
		self.expected_output
	}

	#[must_use]
	pub const fn get_expected_output_max_slippage(&self) -> Option<Decimal> {
		self.expected_output_max_slippage
	}

	#[must_use]
	pub const fn get_expected_output_usd(&self) -> Option<Decimal> {
		self.expected_output_usd
	}

	#[must_use]
	pub const fn get_expected_output_max_slippage_usd(&self) -> Option<Decimal> {
		self.expected_output_max_slippage_usd
	}

	#[must_use]
	pub const fn get_time_estimates(&self) -> Option<&QuoteTimeEstimates> {
		self.time_estimates.as_ref()
	}
}
