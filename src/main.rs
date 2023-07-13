use clap::Parser;

mod config;

const COMMENT_CHAR: char = '#';
const DEFAULT_SEPARATOR: char = '+';

fn main() {
	let cfg = config::Config::parse();
	println!("{cfg:?}");
	println!("{:?}", cfg.addresses());
}
