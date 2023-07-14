use clap::Parser;

mod address;
mod config;

const COMMENT_CHAR: char = '#';
const DEFAULT_SEPARATOR: char = '+';
const KEY_SEPARATOR: char = ':';

fn main() {
	let cfg = config::Config::parse();
	println!("{cfg:?}");
	println!("{:?}", cfg.addresses());
}
