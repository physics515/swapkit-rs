use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::QuoteCallData;
use crate::QuoteEvmTransactionDetails;
use crate::QuoteFee;
use crate::QuoteMeta;
use crate::QuoteSwap;
use crate::QuoteTimeEstimates;
use crate::QuoteTransactionEnum;

/*
{
		"path": "ETH.ETH -> BTC.BTC",
		"providers": [
				"THORCHAIN"
		],
		"subProviders": [
				"THORCHAIN"
		],
		"swaps": HashMap<String, Vec<Vec<QuoteSwap>>>,
		"expectedOutput": "0.04505466",
		"expectedOutputMaxSlippage": "0.04460857425742574257",
		"expectedOutputUSD": "2974.330077737888",
		"expectedOutputMaxSlippageUSD": "2944.881265087018",
		"transaction": QuoteTransaction,
		"optimal": true,
		"complete": true,
		"fees": HashMap<String, Vec<QuoteFee>>,
		"meta": QuoteMeta,
		"inboundAddress": "0xcb4c6ae4f20477f96342467df478ae83512a7bd5",
		"targetAddress": "0xD37BbE5744D730a1d98d8DC97c42F0Ca46aD7146",
		"estimatedTime": 678,
		"calldata": QuoteCallData,
		"contract": "0xD37BbE5744D730a1d98d8DC97c42F0Ca46aD7146",
		"contractMethod": "depositWithExpiry",
		"approvalTarget": "0xD37BbE5744D730a1d98d8DC97c42F0Ca46aD7146",
		"approvalToken": null,
		"evmTransactionDetails": QuoteEvmTransactionDetails,
		"timeEstimates": QuoteTimeEstimates,
		"index": 0
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteRoute {
	#[serde(rename = "path")]
	path: String,

	#[serde(rename = "providers")]
	providers: Vec<String>,

	#[serde(rename = "subProviders")]
	sub_providers: Vec<String>,

	#[serde(rename = "swaps")]
	swaps: HashMap<String, Vec<Vec<QuoteSwap>>>,

	#[serde(rename = "expectedOutput", with = "rust_decimal::serde::str")]
	expected_output: Decimal,

	#[serde(rename = "expectedOutputMaxSlippage", with = "rust_decimal::serde::str")]
	expected_output_max_slippage: Decimal,

	#[serde(rename = "expectedOutputUSD", with = "rust_decimal::serde::str")]
	expected_output_usd: Decimal,

	#[serde(rename = "expectedOutputMaxSlippageUSD", with = "rust_decimal::serde::str")]
	expected_output_max_slippage_usd: Decimal,

	#[serde(rename = "transaction")]
	transaction: QuoteTransactionEnum,

	#[serde(rename = "optimal")]
	optimal: bool,

	#[serde(rename = "complete")]
	complete: bool,

	#[serde(rename = "fees")]
	fees: HashMap<String, Vec<QuoteFee>>,

	#[serde(rename = "meta")]
	meta: QuoteMeta,

	#[serde(rename = "inboundAddress")]
	inbound_address: Option<String>,

	#[serde(rename = "targetAddress")]
	target_address: Option<String>,

	#[serde(rename = "estimatedTime", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	estimated_time: Decimal,

	#[serde(rename = "calldata")]
	calldata: QuoteCallData,

	#[serde(rename = "contract")]
	contract: Option<String>,

	#[serde(rename = "contractMethod")]
	contract_method: Option<String>,

	#[serde(rename = "approvalTarget")]
	approval_target: Option<String>,

	#[serde(rename = "approvalToken")]
	approval_token: Option<String>,

	#[serde(rename = "evmTransactionDetails")]
	evm_transaction_details: QuoteEvmTransactionDetails,

	#[serde(rename = "timeEstimates")]
	time_estimates: QuoteTimeEstimates,

	#[serde(rename = "index", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	index: Decimal,
}

impl QuoteRoute {
	#[must_use]
	pub const fn get_path(&self) -> &String {
		&self.path
	}

	#[must_use]
	pub const fn get_providers(&self) -> &Vec<String> {
		&self.providers
	}

	#[must_use]
	pub const fn get_sub_providers(&self) -> &Vec<String> {
		&self.sub_providers
	}

	#[must_use]
	pub const fn get_swaps(&self) -> &HashMap<String, Vec<Vec<QuoteSwap>>> {
		&self.swaps
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
	pub const fn get_transaction(&self) -> &QuoteTransactionEnum {
		&self.transaction
	}

	#[must_use]
	pub const fn get_optimal(&self) -> bool {
		self.optimal
	}

	#[must_use]
	pub const fn get_complete(&self) -> bool {
		self.complete
	}

	#[must_use]
	pub const fn get_fees(&self) -> &HashMap<String, Vec<QuoteFee>> {
		&self.fees
	}

	#[must_use]
	pub const fn get_meta(&self) -> &QuoteMeta {
		&self.meta
	}

	#[must_use]
	pub const fn get_inbound_address(&self) -> &Option<String> {
		&self.inbound_address
	}

	#[must_use]
	pub const fn get_target_address(&self) -> &Option<String> {
		&self.target_address
	}

	#[must_use]
	pub const fn get_estimated_time(&self) -> Decimal {
		self.estimated_time
	}

	#[must_use]
	pub const fn get_calldata(&self) -> &QuoteCallData {
		&self.calldata
	}

	#[must_use]
	pub const fn get_contract(&self) -> &Option<String> {
		&self.contract
	}

	#[must_use]
	pub const fn get_contract_method(&self) -> &Option<String> {
		&self.contract_method
	}

	#[must_use]
	pub const fn get_approval_target(&self) -> &Option<String> {
		&self.approval_target
	}

	#[must_use]
	pub const fn get_approval_token(&self) -> &Option<String> {
		&self.approval_token
	}

	#[must_use]
	pub const fn get_evm_transaction_details(&self) -> &QuoteEvmTransactionDetails {
		&self.evm_transaction_details
	}

	#[must_use]
	pub const fn get_time_estimates(&self) -> &QuoteTimeEstimates {
		&self.time_estimates
	}

	#[must_use]
	pub const fn get_index(&self) -> Decimal {
		self.index
	}
}
