use serde::{Deserialize, Serialize};

use crate::CurrencyWithDetails;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrenciesWithDetails(Vec<CurrencyWithDetails>);

impl CurrenciesWithDetails {
	#[must_use]
	pub const fn get_currencies(&self) -> &Vec<CurrencyWithDetails> {
		&self.0
	}
}
