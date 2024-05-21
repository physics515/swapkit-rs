use serde::{Deserialize, Serialize};

use crate::TokenIdentifier;

/*
{
	tokens: {
		identifier: string;
	}[];
	metadata?: "true" | "false" | undefined;
	lookup?: "true" | "false" | undefined;
	sparkline?: "true" | "false" | undefined;
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedPricesParameters {
	pub tokens: Vec<TokenIdentifier>,
	pub metadata: Option<bool>,
	pub lookup: Option<bool>,
	pub sparkline: Option<bool>,
}
