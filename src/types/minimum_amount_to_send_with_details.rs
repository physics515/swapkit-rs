use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{InboundAddress, Pool};

/*
{
	"result": "0.00077264",
	"asset": "BTC.BTC",
	"fromInboundAddress": InboundAddress,
	"toInboundAddress": InboundAddress,
	"toPool": Pool,
	"fromPool": Pool
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumAmountToSendWithDetails {
	#[serde(with = "rust_decimal::serde::str")]
	result: Decimal,

	asset: String,

	#[serde(rename = "fromInboundAddress")]
	from_inbound_address: InboundAddress,

	#[serde(rename = "toInboundAddress")]
	to_inbound_address: InboundAddress,

	#[serde(rename = "toPool")]
	to_pool: Pool,

	#[serde(rename = "fromPool")]
	from_pool: Pool,
}

impl MinimumAmountToSendWithDetails {
	#[must_use]
	pub const fn get_result(&self) -> &Decimal {
		&self.result
	}

	#[must_use]
	pub const fn get_asset(&self) -> &str {
		&self.asset
	}

	#[must_use]
	pub const fn get_from_inbound_address(&self) -> &InboundAddress {
		&self.from_inbound_address
	}

	#[must_use]
	pub const fn get_to_inbound_address(&self) -> &InboundAddress {
		&self.to_inbound_address
	}

	#[must_use]
	pub const fn get_to_pool(&self) -> &Pool {
		&self.to_pool
	}

	#[must_use]
	pub const fn get_from_pool(&self) -> &Pool {
		&self.from_pool
	}
}
