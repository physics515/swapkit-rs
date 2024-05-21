use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
		"inboundConfirmationSeconds": 600,
		"outboundDelaySeconds": 348,
		"expectedAmountOut": "0.49226113"
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowQuoteThornodeMeta {
	#[serde(rename = "inboundConfirmationSeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	inbound_confirmation_seconds: Decimal,

	#[serde(rename = "outboundDelaySeconds", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	outbound_delay_seconds: Decimal,

	#[serde(rename = "expectedAmountOut", with = "rust_decimal::serde::str")]
	expected_amount_out: Decimal,
}

impl BorrowQuoteThornodeMeta {
	#[must_use]
	pub const fn get_inbound_confirmation_seconds(&self) -> Decimal {
		self.inbound_confirmation_seconds
	}

	#[must_use]
	pub const fn get_outbound_delay_seconds(&self) -> Decimal {
		self.outbound_delay_seconds
	}

	#[must_use]
	pub const fn get_expected_amount_out(&self) -> Decimal {
		self.expected_amount_out
	}
}
