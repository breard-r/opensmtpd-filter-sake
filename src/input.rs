use crate::address::CodedAddress;
use anyhow::{anyhow, ensure, Result};
use std::str::FromStr;

macro_rules! next_param {
	($it: ident) => {
		$it.next().ok_or(anyhow!("missing parameter"))
	};
}

#[derive(Debug)]
pub struct Input {
	session_id: String,
	token: String,
	address: CodedAddress,
}

impl Input {
	pub fn answer(&self, msg: &str) {
		println!("filter‐result|{0}|{1}|{msg}", self.session_id, self.token);
	}

	pub fn get_coded_address(&self) -> &CodedAddress {
		&self.address
	}
}

pub fn parse_input(input: &str) -> Result<Input> {
	let mut params_it = input.split(crate::PARAM_SEPARATOR);
	let stream = next_param!(params_it)?;
	ensure!(stream == "filter", "{stream}: invalid stream");
	let _version = next_param!(params_it)?;
	let _timestamp = next_param!(params_it)?;
	let _subsystem = next_param!(params_it)?;
	let filter = next_param!(params_it)?;
	ensure!(filter == "rcpt‐to", "{filter}: invalid filter");
	let session_id = next_param!(params_it)?.to_string();
	let token = next_param!(params_it)?.to_string();
	let address = next_param!(params_it)?.trim_end();
	let address = CodedAddress::from_str(address)?;
	Ok(Input {
		session_id,
		token,
		address,
	})
}
