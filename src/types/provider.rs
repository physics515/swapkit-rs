use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/*
{
		"provider": "Woofi",
		"version": {
				"major": 1,
				"minor": 0,
				"patch": 0
		},
		"nbTokens": 4,
		"logo": "https://static.thorswap.net/token-list/images/eth.woo-0x4691937a7508860f876c9c0a2a617e7d9e945d4b.png"
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
	provider: String,

	version: ProviderVersion,

	#[serde(rename = "nbTokens", serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "crate::utils::deserialize_rust_decimal_from_anything")]
	nb_tokens: Decimal,

	logo: String,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderVersion {
	#[serde(serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "crate::utils::deserialize_rust_decimal_from_anything")]
	major: Decimal,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "crate::utils::deserialize_rust_decimal_from_anything")]
	minor: Decimal,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "crate::utils::deserialize_rust_decimal_from_anything")]
	patch: Decimal,
}

impl Provider {
	#[must_use]
	pub const fn get_provider(&self) -> &String {
		&self.provider
	}

	#[must_use]
	pub const fn get_version(&self) -> &ProviderVersion {
		&self.version
	}

	#[must_use]
	pub const fn get_nb_tokens(&self) -> &Decimal {
		&self.nb_tokens
	}

	#[must_use]
	pub const fn get_logo(&self) -> &String {
		&self.logo
	}
}

impl ProviderVersion {
	#[must_use]
	pub const fn get_major(&self) -> &Decimal {
		&self.major
	}

	#[must_use]
	pub const fn get_minor(&self) -> &Decimal {
		&self.minor
	}

	#[must_use]
	pub const fn get_patch(&self) -> &Decimal {
		&self.patch
	}
}
