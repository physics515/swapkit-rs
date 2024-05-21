use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedProviders(Vec<String>);

impl SupportedProviders {
	#[must_use]
	pub const fn get_providers(&self) -> &Vec<String> {
		&self.0
	}
}
