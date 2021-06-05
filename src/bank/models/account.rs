use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountEntry {
	pub id: String,
	pub created_date: String,
	pub modified_date: String,
	pub account_number: String,
	pub trust: String,
}
