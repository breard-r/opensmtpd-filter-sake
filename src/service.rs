use crate::address::KeyedAddress;
use crate::config::Config;
use crate::input::{parse_input, Input};
use anyhow::Result;
use clap::Parser;
use std::collections::HashSet;
use std::io;

pub const BUFF_SIZE: usize = 4096;
pub const CONFIG_END: &str = "config|ready\n";
pub const ANSWER_OK: &str = "proceed";
pub const ANSWER_ERR: &str = "reject|550 No such recipient here";

pub fn start_service() -> Result<()> {
	let cfg = Config::parse();
	let addresses = cfg.addresses()?;
	let mut buffer = String::with_capacity(BUFF_SIZE);
	let stdin = io::stdin();

	// Handshake
	loop {
		buffer.clear();
		stdin.read_line(&mut buffer)?;
		if buffer == CONFIG_END {
			break;
		}
	}
	println!("register|filter|smtp‐in|rcpt‐to");
	println!("register|ready");

	// Input processing
	loop {
		buffer.clear();
		stdin.read_line(&mut buffer)?;
		if buffer.is_empty() {
			continue;
		}
		match parse_input(&buffer) {
			Ok(input) => {
				if allow_email(&input, &addresses) {
					input.answer(ANSWER_OK);
				} else {
					input.answer(ANSWER_ERR);
				}
			}
			Err(e) => {
				eprintln!("error: {e:#}");
			}
		}
	}
}

fn allow_email(input: &Input, addr_lst: &HashSet<KeyedAddress>) -> bool {
	let address = input.get_coded_address();
	for addr_k in addr_lst {
		if addr_k == address {
			return addr_k.check_code(address);
		}
	}
	true
}

#[cfg(test)]
mod tests {
	use super::allow_email;
	use crate::input::parse_input;
	use crate::service::KeyedAddress;
	use std::collections::HashSet;
	use std::str::FromStr;

	fn run_test_with_addr(address: &str) -> bool {
		// Preparing the input
		let input_str = format!("filter|0.5|1576146008.006099|smtp-in|rcpt-to|7641df9771b4ed00|1ef1c203cc576e5d|{address}");
		let input = parse_input(&input_str);
		assert!(input.is_ok());
		let input = input.unwrap();

		// Preparing the test keyed addresses
		let kaddr_lst = [
			"a@example.org:11voiefK5PgCX5F1TTcuoQ==",
			"b:11voiefK5PgCX5F1TTcuoQ==",
		];
		let mut addr_set = HashSet::with_capacity(kaddr_lst.len());
		for kaddr_str in kaddr_lst {
			let keyed_addr = KeyedAddress::from_str(kaddr_str);
			assert!(keyed_addr.is_ok());
			let keyed_addr = keyed_addr.unwrap();
			addr_set.insert(keyed_addr);
		}

		// Run the test
		allow_email(&input, &addr_set)
	}

	#[test]
	#[ignore]
	fn test_valid_code_domain() {
		assert!(run_test_with_addr("a+test+TODO@example.org"));
	}

	#[test]
	#[ignore]
	fn test_valid_code_no_domain() {
		assert!(run_test_with_addr("b+test+TODO@example.org"));
	}

	#[test]
	fn test_invalid_code_domain() {
		assert!(!run_test_with_addr("a+test+orsxg5a@example.org"));
	}

	#[test]
	fn test_invalid_code_no_domain() {
		assert!(!run_test_with_addr("b+test+orsxg5a@example.org"));
	}

	#[test]
	fn test_no_code() {
		assert!(!run_test_with_addr("a+test@example.org"));
	}

	#[test]
	fn test_no_sub_addr() {
		assert!(!run_test_with_addr("b@example.org"));
	}

	#[test]
	fn test_different_domain() {
		assert!(run_test_with_addr("a@example.com"));
	}
}
