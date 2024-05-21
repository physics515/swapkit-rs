use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
	"annualPercentageRate": "2.065459193298301",
	"asset": "ETH.ETH",
	"assetDepth": "1088385398838",
	"assetPrice": "480.79172117979806",
	"assetPriceUSD": "3013.9421721980507",
	"liquidityUnits": "133557978392448",
	"nativeDecimal": "18",
	"poolAPY": "2.065459193298301",
	"runeDepth": "523286689214283",
	"saversAPR": "0.02540880784690256",
	"saversDepth": "729015475877",
	"saversUnits": "672593919670",
	"status": "available",
	"synthSupply": "817537769097",
	"synthUnits": "80331105413335",
	"units": "213889083805783",
	"volume24h": "883753890250918"
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDetail {
	#[serde(rename = "annualPercentageRate", with = "rust_decimal::serde::str")]
	annual_percentage_rate: Decimal,

	asset: String,

	#[serde(rename = "assetDepth", with = "rust_decimal::serde::str")]
	asset_depth: Decimal,

	#[serde(rename = "assetPrice", with = "rust_decimal::serde::str")]
	asset_price: Decimal,

	#[serde(rename = "assetPriceUSD", with = "rust_decimal::serde::str")]
	asset_price_usd: Decimal,

	#[serde(rename = "liquidityUnits", with = "rust_decimal::serde::str")]
	liquidity_units: Decimal,

	#[serde(rename = "nativeDecimal", with = "rust_decimal::serde::str")]
	native_decimal: Decimal,

	#[serde(rename = "poolAPY", with = "rust_decimal::serde::str")]
	pool_apy: Decimal,

	#[serde(rename = "runeDepth", with = "rust_decimal::serde::str")]
	rune_depth: Decimal,

	#[serde(rename = "saversAPR", with = "rust_decimal::serde::str")]
	savers_apr: Decimal,

	#[serde(rename = "saversDepth", with = "rust_decimal::serde::str")]
	savers_depth: Decimal,

	#[serde(rename = "saversUnits", with = "rust_decimal::serde::str")]
	savers_units: Decimal,

	status: String,

	#[serde(rename = "synthSupply", with = "rust_decimal::serde::str")]
	synth_supply: Decimal,

	#[serde(rename = "synthUnits", with = "rust_decimal::serde::str")]
	synth_units: Decimal,

	#[serde(with = "rust_decimal::serde::str")]
	units: Decimal,

	#[serde(rename = "volume24h", with = "rust_decimal::serde::str")]
	volume_24h: Decimal,
}

impl PoolDetail {
	#[must_use]
	pub const fn get_annual_percentage_rate(&self) -> &Decimal {
		&self.annual_percentage_rate
	}

	#[must_use]
	pub const fn get_asset(&self) -> &str {
		&self.asset
	}

	#[must_use]
	pub const fn get_asset_depth(&self) -> &Decimal {
		&self.asset_depth
	}

	#[must_use]
	pub const fn get_asset_price(&self) -> &Decimal {
		&self.asset_price
	}

	#[must_use]
	pub const fn get_asset_price_usd(&self) -> &Decimal {
		&self.asset_price_usd
	}

	#[must_use]
	pub const fn get_liquidity_units(&self) -> &Decimal {
		&self.liquidity_units
	}

	#[must_use]
	pub const fn get_native_decimal(&self) -> &Decimal {
		&self.native_decimal
	}

	#[must_use]
	pub const fn get_pool_apy(&self) -> &Decimal {
		&self.pool_apy
	}

	#[must_use]
	pub const fn get_rune_depth(&self) -> &Decimal {
		&self.rune_depth
	}

	#[must_use]
	pub const fn get_savers_apr(&self) -> &Decimal {
		&self.savers_apr
	}

	#[must_use]
	pub const fn get_savers_depth(&self) -> &Decimal {
		&self.savers_depth
	}

	#[must_use]
	pub const fn get_savers_units(&self) -> &Decimal {
		&self.savers_units
	}

	#[must_use]
	pub const fn get_status(&self) -> &str {
		&self.status
	}

	#[must_use]
	pub const fn get_synth_supply(&self) -> &Decimal {
		&self.synth_supply
	}

	#[must_use]
	pub const fn get_synth_units(&self) -> &Decimal {
		&self.synth_units
	}

	#[must_use]
	pub const fn get_units(&self) -> &Decimal {
		&self.units
	}

	#[must_use]
	pub const fn get_volume_24h(&self) -> &Decimal {
		&self.volume_24h
	}
}
