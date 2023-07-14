use anyhow::{ensure, Error, Result};
use data_encoding::{BASE32_NOPAD, BASE64};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodedAddress {
	local_part: String,
	sub_addr: String,
	code: Vec<u8>,
	domain: Option<String>,
}

impl FromStr for CodedAddress {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let (local_part, domain) = split_local_part(s);
		let parts: Vec<&str> = local_part.split(crate::DEFAULT_SEPARATOR).collect();
		ensure!(parts.len() == 3, "{s}: invalid number of parts");
		let local_part = parts[0].to_string();
		let sub_addr = parts[1].to_string();
		let code = BASE32_NOPAD.decode(parts[2].to_uppercase().as_bytes())?;
		Ok(Self {
			local_part,
			sub_addr,
			code,
			domain,
		})
	}
}

#[derive(Clone, Debug)]
pub struct KeyedAddress {
	local_part: String,
	domain: Option<String>,
	key: Vec<u8>,
}

impl KeyedAddress {
	pub fn check_code(&self, addr: &CodedAddress) -> bool {
		// TODO
		false
	}
}

impl PartialEq for KeyedAddress {
	fn eq(&self, other: &Self) -> bool {
		self.local_part == other.local_part && self.domain == other.domain
	}
}

impl PartialEq<CodedAddress> for KeyedAddress {
	fn eq(&self, other: &CodedAddress) -> bool {
		self.local_part == other.local_part && self.domain == other.domain
	}
}

impl Eq for KeyedAddress {}

impl Hash for KeyedAddress {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.local_part.hash(state);
		self.domain.hash(state);
	}
}

impl FromStr for KeyedAddress {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let ksplit = s.rsplit_once(crate::KEY_SEPARATOR);
		ensure!(ksplit.is_some(), "{s}: key separator not found");
		let (address, key_b64) = ksplit.unwrap();
		let (local_part, domain) = split_local_part(address);
		let key = BASE64.decode(key_b64.as_bytes())?;
		ensure!(!key.is_empty(), "{s}: key cannot be empty");
		Ok(Self {
			local_part,
			domain,
			key,
		})
	}
}

fn split_local_part(s: &str) -> (String, Option<String>) {
	match s.rsplit_once('@') {
		Some((local_part, domain)) => (local_part.to_string(), Some(domain.to_string())),
		None => (s.to_string(), None),
	}
}

#[cfg(test)]
mod tests {
	use super::{CodedAddress, KeyedAddress};
	use std::str::FromStr;

	#[test]
	fn parse_valid_coded_addr_with_domain() {
		let addr_str = "a+test+orsxg5a@example.org";
		let addr = CodedAddress::from_str(addr_str);
		assert!(addr.is_ok(), "unable to parse {addr_str}: {addr:?}");
		let addr = addr.unwrap();
		assert_eq!(addr.local_part, "a");
		assert_eq!(addr.sub_addr, "test");
		assert_eq!(addr.code, b"test");
		assert_eq!(addr.domain, Some("example.org".to_string()));
	}

	#[test]
	fn parse_valid_coded_addr_without_domain() {
		let addr_str = "local.part+test+orsxg5a";
		let addr = CodedAddress::from_str(addr_str);
		assert!(addr.is_ok(), "unable to parse {addr_str}: {addr:?}");
		let addr = addr.unwrap();
		assert_eq!(addr.local_part, "local.part");
		assert_eq!(addr.domain, None);
	}

	#[test]
	fn parse_valid_keyed_addr_with_domain() {
		let addr_str = "a@example.org:11voiefK5PgCX5F1TTcuoQ==";
		let addr = KeyedAddress::from_str(addr_str);
		assert!(addr.is_ok(), "unable to parse {addr_str}: {addr:?}");
		let addr = addr.unwrap();
		assert_eq!(addr.local_part, "a");
		assert_eq!(addr.domain, Some("example.org".to_string()));
		assert_eq!(
			addr.key,
			vec![
				0xd7, 0x5b, 0xe8, 0x89, 0xe7, 0xca, 0xe4, 0xf8, 0x02, 0x5f, 0x91, 0x75, 0x4d, 0x37,
				0x2e, 0xa1
			]
		);
	}

	#[test]
	fn parse_valid_keyed_addr_without_domain() {
		let addr_str = "local.part:3d74YQqk";
		let addr = KeyedAddress::from_str(addr_str);
		assert!(addr.is_ok(), "unable to parse {addr_str}: {addr:?}");
		let addr = addr.unwrap();
		assert_eq!(addr.local_part, "local.part");
		assert_eq!(addr.domain, None);
		assert_eq!(addr.key, vec![0xdd, 0xde, 0xf8, 0x61, 0x0a, 0xa4]);
	}

	#[test]
	fn keyed_addr_empty_address() {
		let res = KeyedAddress::from_str("");
		assert!(res.is_err());
	}

	#[test]
	fn keyed_addr_empty_base64() {
		let res = KeyedAddress::from_str("a:");
		assert!(res.is_err());
	}

	#[test]
	fn keyed_addr_invalid_base64() {
		let res = KeyedAddress::from_str("a:uh2kv%j3");
		assert!(res.is_err());
	}

	#[test]
	fn cmp_coded_addr_with_domain_eq() {
		let addr_1 = CodedAddress::from_str("test+test+orsxg5a@example.org").unwrap();
		let addr_2 = CodedAddress::from_str("test+test+orsxg5a@example.org").unwrap();
		assert_eq!(addr_1, addr_2);
	}

	#[test]
	fn cmp_coded_addr_without_domain_eq() {
		let addr_1 = CodedAddress::from_str("test+test+orsxg5a").unwrap();
		let addr_2 = CodedAddress::from_str("test+test+orsxg5a").unwrap();
		assert_eq!(addr_1, addr_2);
	}

	#[test]
	fn cmp_coded_addr_with_domain_ne() {
		let addr_1 = CodedAddress::from_str("test+test+orsxg5a@example.org").unwrap();
		let addr_2 = CodedAddress::from_str("test2+test+orsxg5a@example.org").unwrap();
		assert_ne!(addr_1, addr_2);
	}

	#[test]
	fn cmp_coded_addr_without_domain_ne() {
		let addr_1 = CodedAddress::from_str("test+test+orsxg5a").unwrap();
		let addr_2 = CodedAddress::from_str("test2+test+orsxg5a").unwrap();
		assert_ne!(addr_1, addr_2);
	}

	#[test]
	fn cmp_keyed_addr_with_domain_eq() {
		let addr_1 = KeyedAddress::from_str("test@example.org:3d74YQqk").unwrap();
		let addr_2 = KeyedAddress::from_str("test@example.org:11voiefK5PgCX5F1TTcuoQ==").unwrap();
		assert_eq!(addr_1, addr_2);
	}

	#[test]
	fn cmp_keyed_addr_without_domain_eq() {
		let addr_1 = KeyedAddress::from_str("test:3d74YQqk").unwrap();
		let addr_2 = KeyedAddress::from_str("test:11voiefK5PgCX5F1TTcuoQ==").unwrap();
		assert_eq!(addr_1, addr_2);
	}

	#[test]
	fn cmp_keyed_addr_with_domain_ne() {
		let addr_1 = KeyedAddress::from_str("test@example.org:3d74YQqk").unwrap();
		let addr_2 = KeyedAddress::from_str("test2@example.org:3d74YQqk").unwrap();
		assert_ne!(addr_1, addr_2);
	}

	#[test]
	fn cmp_keyed_addr_without_domain_ne() {
		let addr_1 = KeyedAddress::from_str("test:3d74YQqk").unwrap();
		let addr_2 = KeyedAddress::from_str("test2:3d74YQqk").unwrap();
		assert_ne!(addr_1, addr_2);
	}

	#[test]
	fn cmp_addr_types_with_domain_eq() {
		let addr_1 = KeyedAddress::from_str("test@example.org:3d74YQqk").unwrap();
		let addr_2 = CodedAddress::from_str("test+test+orsxg5a@example.org").unwrap();
		assert_eq!(addr_1, addr_2);
	}

	#[test]
	fn cmp_addr_types_without_domain_eq() {
		let addr_1 = KeyedAddress::from_str("test:3d74YQqk").unwrap();
		let addr_2 = CodedAddress::from_str("test+test+orsxg5a").unwrap();
		assert_eq!(addr_1, addr_2);
	}

	#[test]
	fn cmp_addr_types_with_domain_ne() {
		let addr_1 = KeyedAddress::from_str("test@example.org:3d74YQqk").unwrap();
		let addr_2 = CodedAddress::from_str("test+test+orsxg5a@example.com").unwrap();
		assert_ne!(addr_1, addr_2);
	}

	#[test]
	fn cmp_addr_types_without_domain_ne() {
		let addr_1 = KeyedAddress::from_str("test:3d74YQqk").unwrap();
		let addr_2 = CodedAddress::from_str("test2+test+orsxg5a").unwrap();
		assert_ne!(addr_1, addr_2);
	}
}