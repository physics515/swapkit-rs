use serde::{Deserialize, Serialize};

use crate::ChainWithDetails;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainsWithDetails(Vec<ChainWithDetails>);

impl ChainsWithDetails {
	#[must_use]
	pub const fn get_chains(&self) -> &Vec<ChainWithDetails> {
		&self.0
	}
}
