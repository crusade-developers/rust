use crate::{Protocol, ServerNode, Validator};

pub mod models {
	use crate::{NodeType, Protocol};
	use serde::{Deserialize, Serialize};

	#[derive(Serialize, Deserialize, Debug)]
	pub struct AccountEntry {
		pub id: String,
		pub created_date: String,
		pub modified_date: String,
		pub account_number: String,
		pub trust: String,
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct PrimaryValidatorConfig {
		/// This will always be [`None`].
		pub primary_validator: Option<()>,
		pub account_number: String,
		pub ip_address: String,
		pub node_identifier: String,
		pub port: Option<u16>,
		pub protocol: Protocol,
		pub version: String,
		pub default_transaction_fee: u32,
		pub root_account_file: String,
		pub root_account_file_hash: String,
		pub seed_block_identifier: String,
		pub daily_confirmation_rate: u32,
		pub node_type: NodeType,
	}
}

#[derive(Debug)]
pub struct PrimaryValidator {
	protocol: Protocol,
	domain: Box<String>,
	client: Box<reqwest::Client>,
}

impl ServerNode for PrimaryValidator {
	type Config = models::PrimaryValidatorConfig;
	type AccountEntry = models::AccountEntry;

	fn protocol(&self) -> Protocol {
		self.protocol
	}

	fn domain(&self) -> Box<String> {
		self.domain.clone()
	}

	fn client(&self) -> Box<reqwest::Client> {
		self.client.clone()
	}

	fn new<T: AsRef<str>>(protocol: Protocol, domain: T) -> Self {
		Self::new_with_client(protocol, domain, Box::new(reqwest::Client::new()))
	}

	fn new_with_client<T: AsRef<str>>(
		protocol: Protocol,
		domain: T,
		client: Box<reqwest::Client>,
	) -> Self {
		Self {
			protocol,
			domain: Box::new(domain.as_ref().to_string()),
			client,
		}
	}
}

impl Validator for PrimaryValidator {}
