pub mod models;

use serde_json::json;

use crate::{
	models::{PaginatedData, PaginationOptions},
	server_node::ServerNode,
	Account, RequestError,
};

use models::{account::AccountEntry, config::BankConfig, transaction::TransactionEntry};

#[derive(Debug)]
/// Used for interacting with the Bank APIs in the TheNewBoston protocol.
pub struct Bank {
	protocol: crate::constants::Protocol,
	domain: Box<String>,
	/// The [`reqwest`] client that is used to send requests.
	client: Box<reqwest::Client>,
}

impl crate::server_node::ServerNode for Bank {
	type Config = BankConfig;
	type AccountEntry = AccountEntry;

	fn new<T: AsRef<str>>(protocol: crate::constants::Protocol, domain: T) -> Self {
		Self::new_with_client(protocol, domain, Box::new(reqwest::Client::new()))
	}

	fn new_with_client<T: AsRef<str>>(
		protocol: crate::constants::Protocol,
		domain: T,
		client: Box<reqwest::Client>,
	) -> Self {
		Self {
			protocol,
			domain: Box::new(domain.as_ref().to_string()),
			client,
		}
	}

	fn protocol(&self) -> crate::constants::Protocol {
		self.protocol
	}

	fn domain(&self) -> Box<String> {
		self.domain.clone()
	}

	fn client(&self) -> Box<reqwest::Client> {
		self.client.clone()
	}
}

impl Bank {
	// TODO: allow entry of account number in search

	/// Gets the paginated list of transactions from the bank server which can
	/// be customized via the [`PaginationOptions`].
	pub async fn get_transactions(
		&self,
		pagination: &PaginationOptions,
	) -> Result<Box<PaginatedData<TransactionEntry>>, RequestError> {
		let query = serde_qs::to_string(pagination)?;
		let res = self
			.client()
			.get(format!(
				"{}?{}",
				self.format_endpoint("/bank_transactions"),
				query
			))
			.send()
			.await?;
		Ok(Box::new(res.json().await?))
	}

	/// Adds the given block to the bank server and thus, hopefully to the
	/// network. Behind the scenes, we generate signatures based upon the
	/// contents of the block message with the [`Account`].
	pub async fn add_block(
		&self,
		account: &Account,
		draft: &models::block::BlockMessageDraft,
	) -> Result<(), RequestError> {
		let signature = account.sign(serde_json::to_string(draft)?.as_bytes());
		let data = &json!({
			"account_number": account.account_number_hex(),
			"message": draft,
			"signature": hex::encode(signature)
		});
		self.client()
			.post(self.format_endpoint("/blocks"))
			.header("content-type", "application/json")
			.body(serde_json::to_string(data)?)
			.send()
			.await?;
		Ok(())
	}
}
