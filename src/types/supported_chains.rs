use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedChains(HashMap<String, String>);

impl SupportedChains {
	#[must_use]
	pub fn get_chains(&self) -> Vec<String> {
		self.0.keys().cloned().collect()
	}

	#[must_use]
	pub fn get_id(&self, chain: &str) -> Option<&String> {
		self.0.get(chain)
	}
}
