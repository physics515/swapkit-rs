use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestABorrowQuoteParams {
	pub asset_in: String,
	pub asset_out: String,
	pub slippage: String,
	pub amount: String,
	pub sender_address: String,
	pub recipient_address: String,
}
