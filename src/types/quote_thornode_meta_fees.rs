use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{Asset, AssetPrice};

/*
{
		"affiliate": {
				"assetAmount": "0.00013583",
				"baseAmount": "13583",
				"decimal": 8
		},
		"asset": {
				"chain": "BTC",
				"symbol": "BTC",
				"ticker": "BTC",
				"type": "Native",
				"network": "Bitcoin",
				"name": "BTC",
				"decimal": 8,
				"isSynth": false
		},
		"outbound": {
				"assetAmount": "0.00007357",
				"baseAmount": "7357",
				"decimal": 8
		},
		"liquidity": {
				"assetAmount": "0.00000588",
				"baseAmount": "588",
				"decimal": 8
		},
		"slippageBps": 1,
		"total": {
				"assetAmount": "0.00021528",
				"baseAmount": "21528",
				"decimal": 8
		},
		"totalBps": 47
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteThornodeMetaFees {
	affiliate: AssetPrice,
	asset: Asset,
	outbound: AssetPrice,
	liquidity: AssetPrice,

	#[serde(rename = "slippageBps", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	slippage_bps: Decimal,

	total: AssetPrice,

	#[serde(rename = "totalBps", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	total_bps: Decimal,
}

impl QuoteThornodeMetaFees {
	#[must_use]
	pub const fn get_affiliate(&self) -> &AssetPrice {
		&self.affiliate
	}

	#[must_use]
	pub const fn get_asset(&self) -> &Asset {
		&self.asset
	}

	#[must_use]
	pub const fn get_outbound(&self) -> &AssetPrice {
		&self.outbound
	}

	#[must_use]
	pub const fn get_liquidity(&self) -> &AssetPrice {
		&self.liquidity
	}

	#[must_use]
	pub const fn get_slippage_bps(&self) -> Decimal {
		self.slippage_bps
	}

	#[must_use]
	pub const fn get_total(&self) -> &AssetPrice {
		&self.total
	}

	#[must_use]
	pub const fn get_total_bps(&self) -> Decimal {
		self.total_bps
	}
}
