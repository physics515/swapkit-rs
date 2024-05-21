use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestASwapQuoteParams {
	pub affiliate_address: Option<String>,
	pub affiliate_basis_points: Option<String>,
	pub buy_asset: String,
	pub is_affiliate_fee_flat: Option<bool>,
	pub recipient_address: String,
	pub sell_amount: String,
	pub sell_asset: String,
	pub sender_address: String,
	pub slippage: Option<String>,
}
