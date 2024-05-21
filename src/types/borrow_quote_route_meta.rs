use serde::{Deserialize, Serialize};

use crate::BorrowQuoteThornodeMeta;

/*
{
		"thornodeMeta": {
				"inboundConfirmationSeconds": 600,
				"outboundDelaySeconds": 348,
				"expectedAmountOut": "0.49226113"
		}
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowQuoteRouteMeta {
	#[serde(rename = "thornodeMeta")]
	thornode_meta: BorrowQuoteThornodeMeta,
}

impl BorrowQuoteRouteMeta {
	#[must_use]
	pub const fn get_thornode_meta(&self) -> &BorrowQuoteThornodeMeta {
		&self.thornode_meta
	}
}
