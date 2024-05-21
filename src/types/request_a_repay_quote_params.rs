use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestARepayQuoteParams {
	pub repay_asset: String,
	pub collateral_asset: String,
	pub amount_percentage: String,
	pub sender_address: String,
	pub collateral_address: String,
	pub affiliate_basis_points: String,
	pub affiliate_address: String,
}
