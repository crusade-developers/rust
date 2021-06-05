use crate::AccountError;
use ed25519_dalek::{Keypair, Signer};
use rand7::rngs::OsRng;
use std::convert::From;

#[derive(Debug)]
pub struct Account {
	pub(crate) pair: Box<Keypair>,
}

impl Account {
	pub fn random() -> Self {
		let mut csprng = OsRng {};
		Self {
			pair: Box::new(Keypair::generate(&mut csprng)),
		}
	}

	pub fn from_signing_key<T: AsRef<str>>(signing_key: T) -> Result<Self, AccountError> {
		use ed25519_dalek_bip32::{PublicKey, SecretKey};

		let bytes = hex::decode(signing_key.as_ref())?;
		let bytes = bytes.as_slice();

		let secret = SecretKey::from_bytes(bytes)?;

		let public = PublicKey::from(&secret);

		Ok(Self {
			pair: Box::new(Keypair { secret, public }),
		})
	}

	pub fn account_number(&self) -> &[u8] {
		self.pair.public.as_bytes()
	}

	pub fn account_number_hex(&self) -> String {
		hex::encode(self.account_number())
	}

	pub fn signing_key(&self) -> &[u8] {
		self.pair.secret.as_bytes()
	}

	pub fn signing_key_hex(&self) -> String {
		hex::encode(self.signing_key())
	}

	pub fn sign(&self, data: &[u8]) -> Box<[u8]> {
		Box::new(self.pair.sign(data).to_bytes())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::AccountError;
	use anyhow::Context;

	#[test]
	fn random_keys() {
		Account::random();
	}

	#[test]
	fn deriving_invalid_hex_secret_throws() {
		match Account::from_signing_key("heloasdf") {
			Err(AccountError::InvalidHex(_)) => return,
			_ => panic!("Should've failed with invalid hex."),
		}
	}

	#[test]
	fn getting_account_key_strings() -> anyhow::Result<()> {
		const SIGNING_KEY: &str =
			"b0ef99e631b34126b0ff138f9efb98df266968feae54ef4842e8b6b04f26cc7d";
		const ACCOUNT_NUMBER: &str =
			"063613f9af31923141239619c076db302044ec39bcb64816667a98117bb81ce4";

		let account = Account::from_signing_key(SIGNING_KEY)
			.context("Failed to create account from signing key.")?;

		assert_eq!(account.signing_key_hex(), SIGNING_KEY);
		assert_eq!(account.account_number_hex(), ACCOUNT_NUMBER);

		Ok(())
	}
}
