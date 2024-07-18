use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{BorrowQuoteRoute, QuoteFee, QuoteSwap, QuoteTimeEstimates, StreamingSwap};

/*
{
		"inboundAddress": "bc1qpqkqxgln9qwahw76nyp3v9fqyc7zy252yhv2kc",
		"inboundConfirmationBlocks": 1,
		"inboundConfirmationSeconds": 600,
		"outboundDelayBlocks": 58,
		"outboundDelaySeconds": 348,
		"fees": HashMap<String, Vec<QuoteFee>>,
		"expiry": 1715973758,
		"warning": "Do not cache this response. Do not send funds after the expiry.",
		"notes": "First output should be to inbound_address, second output should be change back to self, third output should be OP_RETURN, limited to 80 bytes. Do not send below the dust threshold. Do not use exotic spend scripts, locks or address formats (P2WSH with Bech32 address format preferred).",
		"dustThreshold": "10000",
		"memo": "$+:BTC.BTC:bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf:1:t:100",
		"expectedAmountOut": "0.49226113",
		"expectedCollaterizationRatio": "20000",
		"expectedCollateralDeposited": "0.98814537",
		"expectedDebtIssued": "32909.635",
		"recommendedMinAmountIn": "0.00025588",
		"interestRate": 0,
		"streamingSwapBlocks": 4,
		"streamingSwapSeconds": 24,
		"totalOpenLoanSeconds": 972,
		"complete": true,
		"expectedOutput": "0.49226113",
		"expectedOutputMaxSlippage": "0.48981206965174129353",
		"expectedOutputUSD": "30886.8363905783448903382",
		"expectedOutputMaxSlippageUSD": "30733.1705378889003882495021603092742",
		"timeEstimates": QuoteTimeEstimates,
		"swaps": Vec<Vec<QuoteSwap>>,
		"route": BorrowQuoteRoute,
		"amountIn": "1",
		"amountOut": "0.49226113",
		"amountOutMin": "0.48981206965174129353",
		"targetAddress": "bc1qpqkqxgln9qwahw76nyp3v9fqyc7zy252yhv2kc",
		"recipientAddress": "bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf",
		"assetIn": "BTC.BTC",
		"assetOut": "BTC.BTC",
		"estimatedTime": 1008,
		"streamingSwap": StreamingSwap,
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowQuote {
	#[serde(rename = "inboundAddress")]
	inbound_address: Option<String>,

	#[serde(rename = "inboundConfirmationBlocks", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	inbound_confirmation_blocks: Decimal,

	#[serde(rename = "inboundConfirmationSeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	inbound_confirmation_seconds: Decimal,

	#[serde(rename = "outboundDelayBlocks", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	outbound_delay_blocks: Decimal,

	#[serde(rename = "outboundDelaySeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	outbound_delay_seconds: Decimal,

	#[serde(rename = "fees")]
	fees: HashMap<String, Vec<QuoteFee>>,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	expiry: Decimal,

	warning: String,

	notes: String,

	#[serde(rename = "dustThreshold", with = "rust_decimal::serde::str")]
	dust_threshold: Decimal,

	memo: String,

	#[serde(rename = "expectedAmountOut", with = "rust_decimal::serde::str")]
	expected_amount_out: Decimal,

	#[serde(rename = "expectedCollaterizationRatio", with = "rust_decimal::serde::str")]
	expected_collaterization_ratio: Decimal,

	#[serde(rename = "expectedCollateralDeposited", with = "rust_decimal::serde::str")]
	expected_collateral_deposited: Decimal,

	#[serde(rename = "expectedDebtIssued", with = "rust_decimal::serde::str")]
	expected_debt_issued: Decimal,

	#[serde(rename = "recommendedMinAmountIn", with = "rust_decimal::serde::str")]
	recommended_min_amount_in: Decimal,

	#[serde(rename = "interestRate", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	interest_rate: Decimal,

	#[serde(rename = "streamingSwapBlocks", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	streaming_swap_blocks: Decimal,

	#[serde(rename = "streamingSwapSeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	streaming_swap_seconds: Decimal,

	#[serde(rename = "totalOpenLoanSeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	total_open_loan_seconds: Decimal,

	complete: bool,

	#[serde(rename = "expectedOutput", with = "rust_decimal::serde::str")]
	expected_output: Decimal,

	#[serde(rename = "expectedOutputMaxSlippage", with = "rust_decimal::serde::str")]
	expected_output_max_slippage: Decimal,

	#[serde(rename = "expectedOutputUSD", with = "rust_decimal::serde::str")]
	expected_output_usd: Decimal,

	#[serde(rename = "expectedOutputMaxSlippageUSD", with = "rust_decimal::serde::str")]
	expected_output_max_slippage_usd: Decimal,

	#[serde(rename = "timeEstimates")]
	time_estimates: QuoteTimeEstimates,

	swaps: Vec<Vec<QuoteSwap>>,

	route: BorrowQuoteRoute,

	#[serde(rename = "amountIn", with = "rust_decimal::serde::str")]
	amount_in: Decimal,

	#[serde(rename = "amountOut", with = "rust_decimal::serde::str")]
	amount_out: Decimal,

	#[serde(rename = "amountOutMin", with = "rust_decimal::serde::str")]
	amount_out_min: Decimal,

	#[serde(rename = "targetAddress")]
	target_address: String,

	#[serde(rename = "recipientAddress")]
	recipient_address: String,

	#[serde(rename = "assetIn")]
	asset_in: String,

	#[serde(rename = "assetOut")]
	asset_out: String,

	#[serde(rename = "estimatedTime", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	estimated_time: Decimal,

	#[serde(rename = "streamingSwap")]
	streaming_swap: StreamingSwap,
}

impl BorrowQuote {
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
	pub const fn get_dust_threshold(&self) -> Decimal {
		self.dust_threshold
	}

	#[must_use]
	pub fn get_memo(&self) -> &str {
		&self.memo
	}

	#[must_use]
	pub const fn get_expected_amount_out(&self) -> Decimal {
		self.expected_amount_out
	}

	#[must_use]
	pub const fn get_expected_collaterization_ratio(&self) -> Decimal {
		self.expected_collaterization_ratio
	}

	#[must_use]
	pub const fn get_expected_collateral_deposited(&self) -> Decimal {
		self.expected_collateral_deposited
	}

	#[must_use]
	pub const fn get_expected_debt_issued(&self) -> Decimal {
		self.expected_debt_issued
	}

	#[must_use]
	pub const fn get_recommended_min_amount_in(&self) -> Decimal {
		self.recommended_min_amount_in
	}

	#[must_use]
	pub const fn get_interest_rate(&self) -> Decimal {
		self.interest_rate
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
	pub const fn get_total_open_loan_seconds(&self) -> Decimal {
		self.total_open_loan_seconds
	}

	#[must_use]
	pub const fn is_complete(&self) -> bool {
		self.complete
	}

	#[must_use]
	pub const fn get_expected_output(&self) -> Decimal {
		self.expected_output
	}

	#[must_use]
	pub const fn get_expected_output_max_slippage(&self) -> Decimal {
		self.expected_output_max_slippage
	}

	#[must_use]
	pub const fn get_expected_output_usd(&self) -> Decimal {
		self.expected_output_usd
	}

	#[must_use]
	pub const fn get_expected_output_max_slippage_usd(&self) -> Decimal {
		self.expected_output_max_slippage_usd
	}

	#[must_use]
	pub const fn get_time_estimates(&self) -> &QuoteTimeEstimates {
		&self.time_estimates
	}

	#[must_use]
	pub const fn get_swaps(&self) -> &Vec<Vec<QuoteSwap>> {
		&self.swaps
	}

	#[must_use]
	pub const fn get_route(&self) -> &BorrowQuoteRoute {
		&self.route
	}

	#[must_use]
	pub const fn get_amount_in(&self) -> Decimal {
		self.amount_in
	}

	#[must_use]
	pub const fn get_amount_out(&self) -> Decimal {
		self.amount_out
	}

	#[must_use]
	pub const fn get_amount_out_min(&self) -> Decimal {
		self.amount_out_min
	}

	#[must_use]
	pub fn get_target_address(&self) -> &str {
		&self.target_address
	}

	#[must_use]
	pub fn get_recipient_address(&self) -> &str {
		&self.recipient_address
	}

	#[must_use]
	pub fn get_asset_in(&self) -> &str {
		&self.asset_in
	}

	#[must_use]
	pub fn get_asset_out(&self) -> &str {
		&self.asset_out
	}

	#[must_use]
	pub const fn get_estimated_time(&self) -> Decimal {
		self.estimated_time
	}

	#[must_use]
	pub const fn get_streaming_swap(&self) -> &StreamingSwap {
		&self.streaming_swap
	}
}
