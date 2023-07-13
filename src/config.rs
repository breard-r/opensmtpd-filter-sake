use anyhow::Result;
use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
	#[arg(short, long)]
	address: Vec<String>,
	#[arg(short = 'A', long)]
	address_file: Option<PathBuf>,
	#[arg(short, long, default_value_t = crate::DEFAULT_SEPARATOR)]
	separator: char,
}

impl Config {
	pub fn addresses(&self) -> Result<HashSet<String>> {
		let mut addr_set = HashSet::new();
		for addr in &self.address {
			addr_set.insert(addr.to_string());
		}
		if let Some(path) = &self.address_file {
			let f = File::open(path)?;
			let f = BufReader::new(f);
			for line in f.lines() {
				let line = line?;
				let addr = line.trim();
				if !addr.is_empty() && !addr.starts_with(crate::COMMENT_CHAR) {
					addr_set.insert(addr.to_string());
				}
			}
		}
		Ok(addr_set)
	}
}
