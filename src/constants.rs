use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeType {
	Bank,
	PrimaryValidator,
	ConfirmationValidator,
	#[serde(other)]
	#[serde(rename = "")]
	Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
	Http,
	Https,
}
