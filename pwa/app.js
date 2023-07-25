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
	constructor(local_part, separator, domain, key) {
		this.local_part = local_part;
		this.domain = domain;
		this.separator = separator;
		if (Array.isArray(key)) {
			this.key = key;
		} else {
			this.key = base64_decode(key);
		}
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

	register() {
		localStorage.setItem(this.getName(), JSON.stringify(this));
	}
}

document.addEventListener('DOMContentLoaded', () => {
	// Functions to open and close a modal
	function openModal(el) {
		el.classList.add('is-active');
	}

	function openNewAccountModal(el) {
		const new_account_modal = document.querySelector('#modal-add-account');
		openModal(new_account_modal);
	}

	function closeModal(el) {
		if (!(el.id === 'modal-add-account' && localStorage.length === 0)) {
			el.classList.remove('is-active');
		}
	}

	function closeAllModals() {
		(document.querySelectorAll('.modal') || []).forEach((modal) => {
			closeModal(modal);
		});
	}

	// Function to get an account by its name
	function getAccountByName(name) {
		const account_string = localStorage.getItem(name);
		const account_raw = JSON.parse(account_string);
		return new Account(
			account_raw.local_part,
			account_raw.separator,
			account_raw.domain,
			account_raw.key,
		);
	}

	// Function to synchronize the account list
	function syncAccountList() {
		var acc_list = document.querySelector('#account-name');
		while (acc_list.lastElementChild) {
			acc_list.removeChild(acc_list.lastElementChild);
		}
		var account_names = [];
		for (var i = 0, len = localStorage.length; i < len; ++i) {
			account_names.push(localStorage.key(i));
		}
		account_names.sort();
		for (const name of account_names) {
			const account = getAccountByName(name);
			const new_elem = new Option(account.getName(), account.getName());
			acc_list.appendChild(new_elem);
		}
		if (localStorage.length === 0) {
			openNewAccountModal();
		}
	}
	syncAccountList();

	// Add a click event on buttons to open a specific modal
	(document.querySelectorAll('.js-modal-trigger') || []).forEach((trigger) => {
		const modal = trigger.dataset.target;
		const target = document.getElementById(modal);

		trigger.addEventListener('click', () => {
			openModal(target);
		});
	});

	// Add a click event on various child elements to close the parent modal
	(document.querySelectorAll('.modal-background, .modal-close, .modal-card-head .delete, .modal-card-foot .button-close') || []).forEach((close) => {
		const target = close.closest('.modal');

		close.addEventListener('click', () => {
			closeModal(target);
		});
	});

	// Add a keyboard event to close all modals
	document.addEventListener('keydown', (event) => {
		if (event.code === 'Escape') {
			closeAllModals();
		}
	});

	// Add a click event on the new account button to register the new account
	document.querySelector('#btn-new-account').addEventListener('click', (event) => {
		console.log('Adding new account…');
		const new_account = new Account(
			document.querySelector('#new-addr-local-part').value,
			document.querySelector('#new-addr-separator').value,
			document.querySelector('#new-addr-domain').value,
			document.querySelector('#new-addr-key').value,
		);
		console.log(new_account);
		new_account.register();
		console.log(`Account ${new_account.getName()} added.`);
		['#new-addr-local-part', '#new-addr-domain', '#new-addr-key'].forEach((selector) => {
			document.querySelector(selector).value = '';
		});
		document.querySelector('#new-addr-separator').value = '+';
		syncAccountList();
		closeAllModals();
	});

	// Add a click event on the new address button to generate it
	document.querySelector('#btn-generate').addEventListener('click', (event) => {
		event.preventDefault();
		const account = getAccountByName(document.querySelector('#account-name').value);
		const new_address = account.genSubAddr(document.querySelector('#sub-addr-name').value);
		document.querySelector('#generated-addr').value = new_address;
		console.log(`New sub-address for account ${account.getName()}: ${new_address}`);
	});
});

window.addEventListener('load', () => {
	if ('serviceWorker' in navigator) {
		navigator.serviceWorker.register('/sw.js');
	}
});
