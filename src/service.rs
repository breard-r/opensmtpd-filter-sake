use crate::address::CodedAddress;
use crate::config::Config;
use anyhow::Result;
use clap::Parser;
use std::io;

pub const BUFF_SIZE: usize = 4096;
pub const CONFIG_END: &str = "config|ready\n";

pub fn start_service() -> Result<()> {
	let cfg = Config::parse();
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
	println!("register|report|smtp‐in|tx‐rcpt");
	println!("register|ready");

	// Input processing
	loop {
		buffer.clear();
		stdin.read_line(&mut buffer)?;
	}
}
