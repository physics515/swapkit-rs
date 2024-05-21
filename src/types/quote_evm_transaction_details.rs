use serde::{Deserialize, Serialize};

/*
{
		"contractAddress": "0xD37BbE5744D730a1d98d8DC97c42F0Ca46aD7146",
		"contractMethod": "depositWithExpiry",
		"contractParams": [
				"1",
				"0xcb4c6ae4f20477f96342467df478ae83512a7bd5",
				"ETH.ETH",
				"1000000000000000000",
				"=:b:1Lbcfr7sAHTD9CgdQo3HTMTkV8LK4ZnX71:4460857:t:30",
				"1715881803"
		],
		"contractParamsStreaming": [],
		"contractParamsNames": [
				"depositWithExpiry",
				"vault",
				"asset",
				"amount",
				"memo",
				"expiration"
		],
		"approvalToken": null,
		"approvalSpender": "0xD37BbE5744D730a1d98d8DC97c42F0Ca46aD7146"
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteEvmTransactionDetails {
	#[serde(rename = "contractAddress")]
	contract_address: Option<String>,

	#[serde(rename = "contractMethod")]
	contract_method: Option<String>,

	#[serde(rename = "contractParams")]
	contract_params: Vec<Option<String>>,

	#[serde(rename = "contractParamsStreaming")]
	contract_params_streaming: Vec<String>,

	#[serde(rename = "contractParamsNames")]
	contract_params_names: Vec<String>,

	#[serde(rename = "approvalToken")]
	approval_token: Option<String>,

	#[serde(rename = "approvalSpender")]
	approval_spender: Option<String>,
}

impl QuoteEvmTransactionDetails {
	#[must_use]
	pub const fn get_contract_address(&self) -> &Option<String> {
		&self.contract_address
	}

	#[must_use]
	pub const fn get_contract_method(&self) -> &Option<String> {
		&self.contract_method
	}

	#[must_use]
	pub const fn get_contract_params(&self) -> &Vec<Option<String>> {
		&self.contract_params
	}

	#[must_use]
	pub const fn get_contract_params_streaming(&self) -> &Vec<String> {
		&self.contract_params_streaming
	}

	#[must_use]
	pub const fn get_contract_params_names(&self) -> &Vec<String> {
		&self.contract_params_names
	}

	#[must_use]
	pub const fn get_approval_token(&self) -> &Option<String> {
		&self.approval_token
	}

	#[must_use]
	pub const fn get_approval_spender(&self) -> &Option<String> {
		&self.approval_spender
	}
}
