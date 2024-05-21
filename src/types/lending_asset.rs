use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
	"asset": "BTC.BTC",
	"assetDepthAssetAmount": "1103.58594672",
	"runeDepthAssetAmount": "10841279.17868466",
	"loanCr": "200",
	"loanCollateral": "1931.73698979",
	"lendingAvailable": true,
	"filledPercentage": "95.31",
	"derivedDepthPercentage": "9497",
	"ltvPercentage": "50.00"
}
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LendingAsset {
	asset: String,

	#[serde(rename = "assetDepthAssetAmount", with = "rust_decimal::serde::str")]
	asset_depth_asset_amount: Decimal,

	#[serde(rename = "runeDepthAssetAmount", with = "rust_decimal::serde::str")]
	rune_depth_asset_amount: Decimal,

	#[serde(rename = "loanCr", with = "rust_decimal::serde::str")]
	loan_cr: Decimal,

	#[serde(rename = "loanCollateral", with = "rust_decimal::serde::str")]
	loan_collateral: Decimal,

	#[serde(rename = "lendingAvailable")]
	lending_available: bool,

	#[serde(rename = "filledPercentage", with = "rust_decimal::serde::str")]
	filled_percentage: Decimal,

	#[serde(rename = "derivedDepthPercentage", with = "rust_decimal::serde::str")]
	derived_depth_percentage: Decimal,

	#[serde(rename = "ltvPercentage", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "crate::utils::deserialize_rust_decimal_from_anything_option")]
	ltv_percentage: Option<Decimal>,
}

impl LendingAsset {
	#[must_use]
	pub const fn get_asset(&self) -> &str {
		&self.asset
	}

	#[must_use]
	pub const fn get_asset_depth_asset_amount(&self) -> Decimal {
		self.asset_depth_asset_amount
	}

	#[must_use]
	pub const fn get_rune_depth_asset_amount(&self) -> Decimal {
		self.rune_depth_asset_amount
	}

	#[must_use]
	pub const fn get_loan_cr(&self) -> Decimal {
		self.loan_cr
	}

	#[must_use]
	pub const fn get_loan_collateral(&self) -> Decimal {
		self.loan_collateral
	}

	#[must_use]
	pub const fn is_lending_available(&self) -> bool {
		self.lending_available
	}

	#[must_use]
	pub const fn get_filled_percentage(&self) -> Decimal {
		self.filled_percentage
	}

	#[must_use]
	pub const fn get_derived_depth_percentage(&self) -> Decimal {
		self.derived_depth_percentage
	}

	#[must_use]
	pub const fn get_ltv_percentage(&self) -> Option<Decimal> {
		self.ltv_percentage
	}
}
