use std::process::ExitCode;

mod address;
mod config;
mod service;

const COMMENT_CHAR: char = '#';
const DEFAULT_SEPARATOR: char = '+';
const KEY_SEPARATOR: char = ':';

fn main() -> ExitCode {
	match service::start_service() {
		Ok(_) => ExitCode::SUCCESS,
		Err(e) => {
			eprintln!("error: {e:#}");
			ExitCode::FAILURE
		}
	}
}
