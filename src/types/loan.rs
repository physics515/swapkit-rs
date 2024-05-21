use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
	"asset": "BTC.BTC",
	"debtCurrent": "1764.0462",
	"debtIssued": "1764.0462",
	"debtRepaid": "0",
	"collateralCurrent": "0.06489696",
	"collateralDeposited": "0.06489696",
	"collateralWithdrawn": "0",
	"lastOpenHeight": 14876835,
	"lastRepayHeight": 0,
	"ltvPercentage": "39.53",
	"owner": "bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz"
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
	asset: String,

	#[serde(rename = "debtCurrent", with = "rust_decimal::serde::str")]
	debt_current: Decimal,

	#[serde(rename = "debtIssued", with = "rust_decimal::serde::str")]
	debt_issued: Decimal,

	#[serde(rename = "debtRepaid", with = "rust_decimal::serde::str")]
	debt_repaid: Decimal,

	#[serde(rename = "collateralCurrent", with = "rust_decimal::serde::str")]
	collateral_current: Decimal,

	#[serde(rename = "collateralDeposited", with = "rust_decimal::serde::str")]
	collateral_deposited: Decimal,

	#[serde(rename = "collateralWithdrawn", with = "rust_decimal::serde::str")]
	collateral_withdrawn: Decimal,

	#[serde(rename = "lastOpenHeight", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "crate::utils::deserialize_rust_decimal_from_anything")]
	last_open_height: Decimal,

	#[serde(rename = "lastRepayHeight", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "crate::utils::deserialize_rust_decimal_from_anything")]
	last_repay_height: Decimal,

	#[serde(rename = "ltvPercentage", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "crate::utils::deserialize_rust_decimal_from_anything_option")]
	ltv_percentage: Option<Decimal>,

	owner: String,
}

impl Loan {
	#[must_use]
	pub const fn get_asset(&self) -> &str {
		&self.asset
	}

	#[must_use]
	pub const fn get_debt_current(&self) -> Decimal {
		self.debt_current
	}

	#[must_use]
	pub const fn get_debt_issued(&self) -> Decimal {
		self.debt_issued
	}

	#[must_use]
	pub const fn get_debt_repaid(&self) -> Decimal {
		self.debt_repaid
	}

	#[must_use]
	pub const fn get_collateral_current(&self) -> Decimal {
		self.collateral_current
	}

	#[must_use]
	pub const fn get_collateral_deposited(&self) -> Decimal {
		self.collateral_deposited
	}

	#[must_use]
	pub const fn get_collateral_withdrawn(&self) -> Decimal {
		self.collateral_withdrawn
	}

	#[must_use]
	pub const fn get_last_open_height(&self) -> Decimal {
		self.last_open_height
	}

	#[must_use]
	pub const fn get_last_repay_height(&self) -> Decimal {
		self.last_repay_height
	}

	#[must_use]
	pub const fn get_ltv_percentage(&self) -> Option<Decimal> {
		self.ltv_percentage
	}

	#[must_use]
	pub const fn get_owner(&self) -> &str {
		&self.owner
	}
}
