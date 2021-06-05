use serde::{Deserialize, Serialize};

use crate::NodeType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TransactionDraft {
	Fee {
		amount: u32,
		fee: NodeType,
		recipient: String,
	},
	Regular {
		amount: u32,
		memo: String,
		recipient: String,
	},
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockMessageDraft {
	pub balance_key: String,
	#[serde(rename = "txs")]
	pub transactions: Vec<TransactionDraft>,
}
