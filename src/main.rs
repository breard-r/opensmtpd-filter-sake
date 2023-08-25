use std::process::ExitCode;

mod address;
mod code;
mod config;
mod input;
mod service;

const ALLOWED_KEY_SIZES: &[usize] = &[16, 32];
const COMMENT_CHAR: char = '#';
const DEFAULT_SEPARATOR: char = '+';
const KEY_SEPARATOR: char = ':';
const PARAM_SEPARATOR: char = '|';

fn main() -> ExitCode {
	match service::start_service() {
		Ok(_) => ExitCode::SUCCESS,
		Err(e) => {
			eprintln!("error: {e:#}");
			ExitCode::FAILURE
		}
	}
}
