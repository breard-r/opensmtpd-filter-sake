use crate::address::CodedAddress;
use anyhow::{anyhow, ensure, Result};

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
		println!("filter-result|{0}|{1}|{msg}", self.session_id, self.token);
	}

	pub fn get_coded_address(&self) -> &CodedAddress {
		&self.address
	}
}

pub fn parse_input(input: &str, separator: char) -> Result<Input> {
	let mut params_it = input.split(crate::PARAM_SEPARATOR);
	let stream = next_param!(params_it)?;
	ensure!(stream == "filter", "{stream}: invalid stream");
	let version = next_param!(params_it)?;
	ensure!(!version.is_empty(), "empty version");
	let timestamp = next_param!(params_it)?;
	ensure!(!timestamp.is_empty(), "empty timestamp");
	let subsystem = next_param!(params_it)?;
	ensure!(subsystem == "smtp-in", "{subsystem}: invalid subsystem");
	let filter = next_param!(params_it)?;
	ensure!(filter == "rcpt-to", "{filter}: invalid filter");
	let session_id = next_param!(params_it)?.to_string();
	ensure!(!session_id.is_empty(), "empty session id");
	let token = next_param!(params_it)?.to_string();
	ensure!(!token.is_empty(), "empty token");
	let address = next_param!(params_it)?.trim_end();
	let address = CodedAddress::parse(address, separator)?;
	Ok(Input {
		session_id,
		token,
		address,
	})
}

#[cfg(test)]
mod tests {
	use super::parse_input;

	#[test]
	fn test_valid_input() {
		let input = "filter|0.5|1576146008.006099|smtp-in|rcpt-to|7641df9771b4ed00|1ef1c203cc576e5d|derp@example.com";
		let res = parse_input(input, '+');
		assert!(res.is_ok());
		let inp = res.unwrap();
		assert_eq!(inp.session_id, "7641df9771b4ed00");
		assert_eq!(inp.token, "1ef1c203cc576e5d");
	}

	#[test]
	fn test_empty_input() {
		assert!(parse_input("", '+').is_err());
	}

	#[test]
	fn test_empty_stream() {
		let input = "|0.5|1576146008.006099|smtp-in|rcpt-to|7641df9771b4ed00|1ef1c203cc576e5d|derp@example.com";
		assert!(parse_input(input, '+').is_err());
	}

	#[test]
	fn test_invalid_stream() {
		let input = "invalid|0.5|1576146008.006099|smtp-in|rcpt-to|7641df9771b4ed00|1ef1c203cc576e5d|derp@example.com";
		assert!(parse_input(input, '+').is_err());
	}

	#[test]
	fn test_empty_version() {
		let input = "filter||1576146008.006099|smtp-in|rcpt-to|7641df9771b4ed00|1ef1c203cc576e5d|derp@example.com";
		assert!(parse_input(input, '+').is_err());
	}

	#[test]
	fn test_empty_timestamp() {
		let input =
			"filter|0.5||smtp-in|rcpt-to|7641df9771b4ed00|1ef1c203cc576e5d|derp@example.com";
		assert!(parse_input(input, '+').is_err());
	}

	#[test]
	fn test_empty_subsystem() {
		let input = "filter|0.5|1576146008.006099||rcpt-to|7641df9771b4ed00|1ef1c203cc576e5d|derp@example.com";
		assert!(parse_input(input, '+').is_err());
	}

	#[test]
	fn test_invalid_subsystem() {
		let input = "filter|0.5|1576146008.006099|invalid|rcpt-to|7641df9771b4ed00|1ef1c203cc576e5d|derp@example.com";
		assert!(parse_input(input, '+').is_err());
	}

	#[test]
	fn test_empty_phase() {
		let input = "filter|0.5|1576146008.006099|smtp-in||7641df9771b4ed00|1ef1c203cc576e5d|derp@example.com";
		assert!(parse_input(input, '+').is_err());
	}

	#[test]
	fn test_invalid_phase() {
		let input = "filter|0.5|1576146008.006099|smtp-in|invalid|7641df9771b4ed00|1ef1c203cc576e5d|derp@example.com";
		assert!(parse_input(input, '+').is_err());
	}

	#[test]
	fn test_empty_id() {
		let input =
			"filter|0.5|1576146008.006099|smtp-in|rcpt-to||1ef1c203cc576e5d|derp@example.com";
		assert!(parse_input(input, '+').is_err());
	}

	#[test]
	fn test_empty_token() {
		let input =
			"filter|0.5|1576146008.006099|smtp-in|rcpt-to|7641df9771b4ed00||derp@example.com";
		assert!(parse_input(input, '+').is_err());
	}

	#[test]
	fn test_empty_data() {
		let input =
			"filter|0.5|1576146008.006099|smtp-in|rcpt-to|7641df9771b4ed00|1ef1c203cc576e5d|";
		assert!(parse_input(input, '+').is_err());
	}

	#[test]
	fn test_missing_data() {
		let input =
			"filter|0.5|1576146008.006099|smtp-in|rcpt-to|7641df9771b4ed00|1ef1c203cc576e5d";
		assert!(parse_input(input, '+').is_err());
	}
}
