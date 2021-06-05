use crate::{models::PaginationOptions, server_node::ServerNode, RequestError};
use async_trait::async_trait;

pub mod confirmation_validator;
pub mod primary_validator;

pub mod models {
	use serde::{Deserialize, Serialize};

	#[derive(Serialize, Deserialize, Debug)]
	pub struct AccountEntry {
		pub id: String,
		pub account_number: String,
		pub balance: u32,
		pub balance_lock: String,
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct AccountBalanceRes {
		pub balance: Option<u32>,
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct AccountBalanceLockRes {
		pub balance_lock: Option<String>,
	}
}

#[async_trait]
pub trait Validator: ServerNode {
	async fn get_accounts(
		&self,
		pagination: &PaginationOptions,
	) -> Result<Box<crate::models::PaginatedData<models::AccountEntry>>, RequestError> {
		let query_string = serde_qs::to_string(pagination)?;
		println!("{:?}", query_string);
		let res = self
			.client()
			.get(format!(
				"{}?{}",
				self.format_endpoint("/accounts"),
				query_string
			))
			.send()
			.await?;
		Ok(Box::new(res.json().await?))
	}

	async fn get_account_balance<T: AsRef<str> + Send>(
		&self,
		account_number: T,
	) -> Result<Option<u32>, RequestError> {
		let res = self
			.client()
			.get(self.format_endpoint(format!("/accounts/{}/balance", account_number.as_ref())))
			.send()
			.await?;
		let data: models::AccountBalanceRes = res.json().await?;
		Ok(data.balance)
	}

	async fn get_account_balance_lock<T: AsRef<str> + Send>(
		&self,
		account_number: T,
	) -> Result<Option<String>, RequestError> {
		let res = self
			.client()
			.get(self.format_endpoint(format!(
				"/accounts/{}/balance_lock",
				account_number.as_ref()
			)))
			.send()
			.await?;
		let data: models::AccountBalanceLockRes = res.json().await?;
		Ok(data.balance_lock)
	}
}
