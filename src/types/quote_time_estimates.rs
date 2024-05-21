use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
		"inboundMs": 24000,
		"outboundMs": 654000,
		"streamingMs": 0,
		"swapMs": 6000
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteTimeEstimates {
	#[serde(rename = "inboundMs", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	inbound_ms: Decimal,

	#[serde(rename = "outboundMs", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	outbound_ms: Decimal,

	#[serde(rename = "streamingMs", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	streaming_ms: Decimal,

	#[serde(rename = "swapMs", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	swap_ms: Decimal,
}

impl QuoteTimeEstimates {
	#[must_use]
	pub const fn get_inbound_ms(&self) -> Decimal {
		self.inbound_ms
	}

	#[must_use]
	pub const fn get_outbound_ms(&self) -> Decimal {
		self.outbound_ms
	}

	#[must_use]
	pub const fn get_streaming_ms(&self) -> Decimal {
		self.streaming_ms
	}

	#[must_use]
	pub const fn get_swap_ms(&self) -> Decimal {
		self.swap_ms
	}
}
