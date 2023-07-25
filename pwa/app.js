function base32_nopad_encode(slice) {
	const encoder = new base32.Encoder({ type: "rfc4648", lc: true });
	const code = encoder.write(slice).finalize();
	return code.replaceAll('=', '');
}

function base64_decode(str_b64) {
	const raw_str = atob(str_b64);
	const length = raw_str.length;
	var b = [];
	for (var i = 0; i < length; i++) {
		b.push(raw_str.charCodeAt(i));
	}
	return b;
}

class Account {
	constructor(local_part, separator, domain, key_b64) {
		this.local_part = local_part;
		this.domain = domain;
		this.separator = separator;
		this.key = base64_decode(key_b64);
	}

	getName() {
		return `${this.local_part}@${this.domain}`;
	}

	genSubAddr(sub_addr_name) {
		var hasher = sha256.hmac.create(this.key);
		hasher.update(this.local_part);
		hasher.update(this.separator);
		hasher.update(sub_addr_name);
		const hash = hasher.array();
		const offset = hash[hash.length - 1] & 0xf;
		const reduced_hash = hash.slice(offset, offset + 5);
		const code = base32_nopad_encode(reduced_hash);
		return `${this.local_part}${this.separator}${sub_addr_name}${this.separator}${code}@${this.domain}`
	}
}

['a', 'b'].forEach((e) => {
	const test_addr = new Account(e, '+', 'example.org', '11voiefK5PgCX5F1TTcuoQ==');
	console.log(test_addr);
	console.log('Account name: ' + test_addr.getName());
	console.log('Sub-addr: ' + test_addr.genSubAddr('test'));
});

window.addEventListener('load', () => {
	if ('serviceWorker' in navigator) {
		navigator.serviceWorker.register('/sw.js');
	}
});
