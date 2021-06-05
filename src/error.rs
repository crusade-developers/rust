use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("account failed")]
	AccountError(#[from] AccountError),
	#[error("request failed")]
	RequestError(#[from] RequestError),
}

#[derive(Error, Debug)]
pub enum AccountError {
	#[error("the hexadecimal string was invalid")]
	InvalidHex(#[from] hex::FromHexError),
	#[error("invalid key size")]
	InvalidKeys(#[from] ed25519_dalek::SignatureError),
}

#[derive(Error, Debug)]
pub enum RequestError {
	#[error("the request failed")]
	RequestFailed(#[from] reqwest::Error),
	#[error("unable to create the query string")]
	InvalidQueryString(#[from] serde_qs::Error),
	#[error("failed to serialize struct into json")]
	InvalidJson(#[from] serde_json::Error),
}
