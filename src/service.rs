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
