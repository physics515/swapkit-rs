use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{Asset, AssetPrice, Depth, PoolDetail};

/*
{
		"asset": Asset,
		"runeDepth": Depth,
		"assetDepth": Depth,
		"assetPriceUSD": AssetPrice,
		"assetPrice": AssetPrice,
		"liquidityUnits": AssetPrice,
		"poolAPY": "2.065459193298301",
		"saversDepth": AssetPrice,
		"saversUnits": AssetPrice,
		"status": "available",
		"synthSupply": AssetPrice,
		"synthUnits": AssetPrice,
		"units": AssetPrice,
		"volume24h": AssetPrice,
		"detail": PoolDetails
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
	asset: Asset,

	#[serde(rename = "runeDepth")]
	rune_depth: Depth,

	#[serde(rename = "assetDepth")]
	asset_depth: Depth,

	#[serde(rename = "assetPriceUSD")]
	asset_price_usd: AssetPrice,

	#[serde(rename = "assetPrice")]
	asset_price: AssetPrice,

	#[serde(rename = "liquidityUnits")]
	liquidity_units: AssetPrice,

	#[serde(rename = "poolAPY", with = "rust_decimal::serde::str")]
	pool_apy: Decimal,

	#[serde(rename = "saversDepth")]
	savers_depth: AssetPrice,

	#[serde(rename = "saversUnits")]
	savers_units: AssetPrice,

	status: String,

	#[serde(rename = "synthSupply")]
	synth_supply: AssetPrice,

	#[serde(rename = "synthUnits")]
	synth_units: AssetPrice,

	units: AssetPrice,

	#[serde(rename = "volume24h")]
	volume_24h: AssetPrice,

	detail: PoolDetail,
}

impl Pool {
	#[must_use]
	pub const fn get_asset(&self) -> &Asset {
		&self.asset
	}

	#[must_use]
	pub const fn get_rune_depth(&self) -> &Depth {
		&self.rune_depth
	}

	#[must_use]
	pub const fn get_asset_depth(&self) -> &Depth {
		&self.asset_depth
	}

	#[must_use]
	pub const fn get_asset_price_usd(&self) -> &AssetPrice {
		&self.asset_price_usd
	}

	#[must_use]
	pub const fn get_asset_price(&self) -> &AssetPrice {
		&self.asset_price
	}

	#[must_use]
	pub const fn get_liquidity_units(&self) -> &AssetPrice {
		&self.liquidity_units
	}

	#[must_use]
	pub const fn get_pool_apy(&self) -> Decimal {
		self.pool_apy
	}

	#[must_use]
	pub const fn get_savers_depth(&self) -> &AssetPrice {
		&self.savers_depth
	}

	#[must_use]
	pub const fn get_savers_units(&self) -> &AssetPrice {
		&self.savers_units
	}

	#[must_use]
	pub fn get_status(&self) -> &str {
		&self.status
	}

	#[must_use]
	pub const fn get_synth_supply(&self) -> &AssetPrice {
		&self.synth_supply
	}

	#[must_use]
	pub const fn get_synth_units(&self) -> &AssetPrice {
		&self.synth_units
	}

	#[must_use]
	pub const fn get_units(&self) -> &AssetPrice {
		&self.units
	}

	#[must_use]
	pub const fn get_volume_24h(&self) -> &AssetPrice {
		&self.volume_24h
	}

	#[must_use]
	pub const fn get_detail(&self) -> &PoolDetail {
		&self.detail
	}
}
