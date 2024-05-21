use serde::{Deserialize, Serialize};

/*
{
	identifier: string;
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenIdentifier {
	pub identifier: String,
}
