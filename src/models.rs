use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedData<T> {
	pub count: u32,
	pub next: Option<String>,
	pub previous: Option<String>,
	pub results: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
/// When these values are [`None`], they will not be included within the url.
/// Therefore, the server will use the default value specified by the
/// maintainer.
pub struct PaginationOptions {
	pub limit: Option<u32>,
	pub offset: Option<u32>,
}

impl std::default::Default for PaginationOptions {
	fn default() -> Self {
		Self {
			limit: None,
			offset: None,
		}
	}
}
