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

class AccountValueError extends Error {
	constructor(field_id, ...params) {
		super(...params);
		if (Error.captureStackTrace) {
			Error.captureStackTrace(this, AccountValueError);
		}
		this.name = "AccountValueError";
		this.field_id = field_id;
	}
}

class Account {
	constructor(local_part, separator, domain, key) {
		// Set the local part
		if (!local_part) {
			throw new AccountValueError("new-addr-local-part", "The local part cannot be empty.");
		}
		this.local_part = local_part;

		// Set the separator
		if (!separator || separator.length !== 1) {
			throw new AccountValueError("new-addr-separator", "The separator must be a single character.");
		}
		this.separator = separator;

		// Set the domain name
		if (!domain) {
			throw new AccountValueError("new-addr-domain", "The domain cannot be empty.");
		}
		this.domain = domain;

		// Set the private key
		if (Array.isArray(key)) {
			this.key = key;
		} else {
			try {
				this.key = base64_decode(key);
			} catch (e) {
				console.log(e);
				throw new AccountValueError("new-addr-key", "The key must be a valid base64 string.");
			}
			if (!this.key.length) {
				throw new AccountValueError("new-addr-key", "The key must not be empty.");
			}
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
		// Handle duplicate
		if (localStorage.getItem(this.getName())) {
			throw new AccountValueError("", "The account already exists.");
		}
		localStorage.setItem(this.getName(), JSON.stringify(this));
	}
}

document.addEventListener('DOMContentLoaded', () => {
	// Functions to display and remove the error message
	function setErrorMessage(message) {
		unsetErrorMessage();
		const el = document.createElement('p');
		el.classList.add('notification');
		el.classList.add('is-danger');
		el.classList.add('is-light');
		el.appendChild(document.createTextNode(message));
		document.querySelector('#new-account-error-msg-container').appendChild(el);
	}

	function unsetErrorMessage() {
		const el_cont = document.querySelector('#new-account-error-msg-container');
		while (el_cont.lastElementChild) {
			el_cont.removeChild(el_cont.lastElementChild);
		}
		['#new-addr-local-part', '#new-addr-separator', '#new-addr-domain', '#new-addr-key'].forEach((selector) => {
			document.querySelector(selector).classList.remove('is-danger');
		});
	}

	// Functions to open and close a modal
	function openModal(el) {
		if (el.id === 'modal-del-account') {
			const name_el = document.createTextNode(getSelectedAccountName());
			document.querySelector('#del-account-name').appendChild(name_el);
		}
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

	// Function to get the name of the currently selected account
	function getSelectedAccountName() {
		return document.querySelector('#account-name').value;
	}

	// Function to get an account by its name
	function getAccountByName(name) {
		const account_string = localStorage.getItem(name);
		if (!account_string) {
			return null;
		}
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

	// Add a click event on the delete account button to display the confirmation message
	document.querySelector('#btn-del-account-confirm').addEventListener('click', (event) => {
		const account_name = getSelectedAccountName();
		localStorage.removeItem(account_name);
		console.log(`Account ${account_name} deleted.`);
		syncAccountList();
		closeAllModals();
	});

	// Add a click event on the new account button to register the new account
	document.querySelector('#btn-new-account').addEventListener('click', (event) => {
		console.log('Adding new account…');
		try {
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
		} catch (e) {
			console.log(`${e.name}: ${e.field_id}: ${e.message}`);
			setErrorMessage(e.message);
			if (e.field_id) {
				document.getElementById(e.field_id).classList.add('is-danger');
			}
		}
	});

	// Add a click event on the new address button to generate it
	document.querySelector('#btn-generate').addEventListener('click', (event) => {
		event.preventDefault();
		const account = getAccountByName(document.querySelector('#account-name').value);
		const new_address = account.genSubAddr(document.querySelector('#sub-addr-name').value);
		document.querySelector('#generated-addr').value = new_address;
		console.log(`New sub-address for account ${account.getName()}: ${new_address}`);
	});

	// Add a change event on the main form to remove previously generated address
	['#account-name', '#sub-addr-name'].forEach((selector) => {
		document.querySelector(selector).addEventListener('change', () => {
			document.querySelector('#generated-addr').value = '';
		});
	});
});

window.addEventListener('load', () => {
	if ('serviceWorker' in navigator) {
		navigator.serviceWorker.register('/sw.js');
	}
});
