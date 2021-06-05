use serde::{Deserialize, Serialize};

use crate::NodeType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockData {
	pub id: String,
	pub created_date: String,
	pub modified_date: String,
	pub balance_key: String,
	pub sender: String,
	pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionEntry {
	pub id: String,
	pub block: BlockData,
	pub amount: u32,
	pub fee: NodeType,
	pub memo: String,
	pub recipient: String,
}
