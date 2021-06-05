use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorEntry {
	pub account_number: String,
	pub ip_address: String,
	pub node_identifier: String,
	pub port: Option<u16>,
	pub protocol: crate::constants::Protocol,
	pub version: String,
	pub default_transaction_fee: u32,
	pub root_account_file: String,
	pub root_account_file_hash: String,
	pub seed_block_identifier: String,
	pub daily_confirmation_rate: u32,
	pub trust: String,
}
