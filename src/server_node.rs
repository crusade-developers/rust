use crate::{
	models::{PaginatedData, PaginationOptions},
	Protocol, RequestError,
};
use async_trait::async_trait;

pub mod models {
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
		pub daily_confirmation_rate: Option<u32>,
		pub trust: String,
	}
}

#[async_trait]
pub trait ServerNode {
	type Config: serde::de::DeserializeOwned;
	type AccountEntry: serde::de::DeserializeOwned;

	fn new<T: AsRef<str>>(protocol: Protocol, domain: T) -> Self;
	fn new_with_client<T: AsRef<str>>(
		protocol: Protocol,
		domain: T,
		client: Box<reqwest::Client>,
	) -> Self;

	fn protocol(&self) -> Protocol;
	fn domain(&self) -> Box<String>;
	fn client(&self) -> Box<reqwest::Client>;

	fn format_endpoint<T: AsRef<str>>(&self, endpoint: T) -> String {
		use Protocol::*;
		format!(
			"{}://{}{}",
			match self.protocol() {
				Http => "http",
				Https => "https",
			},
			self.domain(),
			endpoint.as_ref()
		)
	}

	async fn get_config(&self) -> Result<Box<Self::Config>, RequestError> {
		let res = self
			.client()
			.get(self.format_endpoint("/config"))
			.send()
			.await?;
		Ok(Box::new(res.json().await?))
	}

	async fn get_validators(
		&self,
		pagination: &PaginationOptions,
	) -> Result<Box<PaginatedData<models::ValidatorEntry>>, RequestError> {
		let query_string = serde_qs::to_string(pagination)?;
		let res = self
			.client()
			.get(format!(
				"{}?{}",
				self.format_endpoint("/validators"),
				query_string
			))
			.send()
			.await?;
		Ok(Box::new(res.json().await?))
	}

	async fn get_accounts(&self) -> Result<Box<PaginatedData<Self::AccountEntry>>, RequestError> {
		let res = self
			.client()
			.get(self.format_endpoint("/accounts"))
			.send()
			.await?;
		Ok(Box::new(res.json().await?))
	}
}
