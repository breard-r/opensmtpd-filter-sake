use hmac::{Hmac, Mac};
use sha2::Sha256;

const NB_BYTES: usize = 5;

pub fn generate_code(local_part: &str, separator: char, sub_addr: &str, key: &[u8]) -> Vec<u8> {
	// Compute the HMAC-SHA-256
	let mut hmac = Hmac::<Sha256>::new_from_slice(key).unwrap();
	hmac.update(local_part.as_bytes());
	hmac.update(separator.to_string().as_bytes());
	hmac.update(sub_addr.as_bytes());
	let result = hmac.finalize().into_bytes();

	// Reduce the result to NB_BYTES using a dynamic offset truncation
	let offset = (result[result.len() - 1] & 0xf) as usize;
	result[offset..offset + NB_BYTES].to_vec()
}

#[cfg(test)]
mod tests {
	use super::generate_code;

	#[test]
	fn code_generation() {
		let key: &[u8] = &[
			0xd7, 0x5b, 0xe8, 0x89, 0xe7, 0xca, 0xe4, 0xf8, 0x02, 0x5f, 0x91, 0x75, 0x4d, 0x37,
			0x2e, 0xa1,
		];
		let code = generate_code("a", '+', "test", key);
		assert_eq!(code, vec![0x7d, 0xd8, 0xd7, 0x1c, 0x8e]);
	}
}
