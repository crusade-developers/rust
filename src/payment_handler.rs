use crate::{
	bank::models::{
		block::{BlockMessageDraft, TransactionDraft},
		config::BankConfig,
	},
	Account, Bank, NodeType, PrimaryValidator, Protocol, RequestError, ServerNode, Validator,
};

#[derive(Debug)]
pub struct PaymentHandler {
	client: Box<reqwest::Client>,
	bank: Box<Bank>,
	bank_config: Option<Box<BankConfig>>,
}

impl PaymentHandler {
	pub fn new<T: Into<String>>(bank_url: T) -> Self {
		let client = Box::new(reqwest::Client::new());
		Self {
			client: client.clone(), // cloning Box not client
			bank: Box::new(Bank::new_with_client(
				Protocol::Http,
				bank_url.into(),
				client,
			)),
			bank_config: None,
		}
	}

	pub async fn init(&mut self) -> Result<(), RequestError> {
		let config = self.bank.get_config().await?;
		self.bank_config = Some(config);
		Ok(())
	}

	pub async fn send_coins<T: Into<String>, F: Into<String>>(
		&self,
		sender: &Account,
		recipient: T,
		amount: u32,
		memo: Option<F>,
	) -> Result<(), RequestError> {
		let bank_config = self
			.bank_config
			.clone()
			.expect("Failed to load bank config. Did you forget to init?");

		let validator = PrimaryValidator::new_with_client(
			Protocol::Http,
			bank_config.primary_validator.ip_address,
			self.client.clone(),
		);

		self.bank
			.add_block(
				sender,
				&BlockMessageDraft {
					transactions: vec![
						TransactionDraft::Regular {
							recipient: recipient.into(),
							memo: match memo {
								Some(val) => val.into(),
								_ => String::new(),
							},
							amount,
						},
						TransactionDraft::Fee {
							recipient: bank_config.account_number,
							fee: NodeType::Bank,
							amount: bank_config.default_transaction_fee,
						},
						TransactionDraft::Fee {
							recipient: bank_config.primary_validator.account_number,
							fee: NodeType::PrimaryValidator,
							amount: bank_config.primary_validator.default_transaction_fee,
						},
					],
					balance_key: validator
						.get_account_balance_lock(sender.account_number_hex())
						.await?
						.unwrap(),
				},
			)
			.await?;

		Ok(())
	}
}
