use serde::{Deserialize, Serialize};

/*
{
		"warningCode": "5003",
		"warningMessage": "Default slippage is used for this quote"
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteWarning {
	#[serde(rename = "warningCode")]
	warning_code: String,

	#[serde(rename = "warningMessage")]
	warning_message: String,
}

impl QuoteWarning {
	#[must_use]
	pub const fn get_warning_code(&self) -> &String {
		&self.warning_code
	}

	#[must_use]
	pub const fn get_warning_message(&self) -> &String {
		&self.warning_message
	}
}
