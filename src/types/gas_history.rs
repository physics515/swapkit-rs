use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
	"lastTimestamp": 1688396693338,
	"chainId": "43114",
	"unitName": "wei",
	"history": [
		{
			"value": 25,
			"timestamp": 1688394892940
		}
	],
	"average24h": 25.01171800476548,
	"average7d": 25.46623886101168
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasHistory {
	#[serde(rename = "lastTimestamp", serialize_with = "rust_decimal::serde::str::serialize")]
	last_timestamp: Decimal,

	#[serde(rename = "chainId")]
	chain_id: String,

	#[serde(rename = "unitName")]
	unit_name: String,

	history: Vec<GasHistoryItem>,

	#[serde(rename = "average24h", serialize_with = "rust_decimal::serde::str::serialize")]
	average_24h: Decimal,

	#[serde(rename = "average7d", serialize_with = "rust_decimal::serde::str::serialize")]
	average_7d: Decimal,
}

impl GasHistory {
	#[must_use]
	pub const fn get_last_timestamp(&self) -> &Decimal {
		&self.last_timestamp
	}

	#[must_use]
	pub const fn get_chain_id(&self) -> &String {
		&self.chain_id
	}

	#[must_use]
	pub const fn get_unit_name(&self) -> &String {
		&self.unit_name
	}

	#[must_use]
	pub const fn get_history(&self) -> &Vec<GasHistoryItem> {
		&self.history
	}

	#[must_use]
	pub const fn get_average_24h(&self) -> &Decimal {
		&self.average_24h
	}

	#[must_use]
	pub const fn get_average_7d(&self) -> &Decimal {
		&self.average_7d
	}
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasHistoryItem {
	#[serde(serialize_with = "rust_decimal::serde::str::serialize")]
	value: Decimal,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize")]
	timestamp: Decimal,
}

impl GasHistoryItem {
	#[must_use]
	pub const fn get_value(&self) -> &Decimal {
		&self.value
	}

	#[must_use]
	pub const fn get_timestamp(&self) -> &Decimal {
		&self.timestamp
	}
}
