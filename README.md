# Sub-Address KEy (SAKE) filter

[![Build Status](https://github.com/breard-r/opensmtpd-filter-sake/actions/workflows/ci.yml/badge.svg)](https://github.com/breard-r/opensmtpd-filter-sake/actions/workflows/ci.yml)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.64.0+-lightgray.svg)
![License MIT OR Apache 2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)

Sub-address key filter for OpenSMTPD.


## What is the purpose of this project?

When giving your email address to someone, for example when registering an account on a website, a good practice is to give an unique address for each recipient. This way, if you start receiving spam on this unique address, you know which service has leaked your email address. One common way to do so is to use the sub-address delimiter (by default, the character `+`) in order to add a part with the service's name.

The problem is, people know that whatever is after the `+` can be discarded, and therefore some services drops it. This can also happen after a leak if the spammer doesn't want you to know which website has been breached. Furthermore, a spammer could also add a custom part after the `+` in order to cover its tracks.

Changing the default sub-address delimiter is a good idea, but isn't completely secure: in most cases, anyone will see the pattern you are using and will be able to deduce your email address for other services. For instance, if someone knows that you registered on `www.acme-corp.example.com` using the address `darra.acme-corp@mail.example.org` and on `www.super-social.example.com` using the address `darra.super-social@mail.example.org`, this person will deduce that your address on any service named `x` will be `darra.x@mail.example.org`.

This filter adds a way to configure some addresses (or aliases) in a way that the part after the sub-address delimiter includes a verification code that cannot be guessed. Following the previous example using the dot instead of the plus character as a sub-address delimiter, the addresses could be `darra.acme-corp.nbvtenby@mail.example.org` and `darra.super-social.heywkmrx@mail.example.org`. As you can see, a 5 bytes code in base32 has been added after the second delimiter.

This verification code is derived from both the address itself and a private key that only you and your email server know. Any email sent to this address without a valid authentication code will be rejected with `550 No such recipient here`. Therefore, one must know the private key in order to generate new valid addresses, which means only you can do so.

For more information on how to use it, please read the FAQ.


## Building

```
cargo build --release
```

The executable should be located in the `./target/release/` directory.


## Usage

The filter accepts the following options:

- `--address` or `-a`: specify an address where the filter will enforce the presence of a valid verification code (see below for the format)
- `--address-file` or `-A`: the path to a file where each line is an address as specified in `--address`
- `--separator` or `-s`: set the sub-address delimiter character (default: `+`) : this must match the character defined in `smtpd.conf` using `smtp sub-addr-delim`

An address must be composed of the following elements:
- the local part
- (optional) an `@` followed by a domain name
- the `:` character
- the private key in base64 (with padding)

Specifying a domain name configures the filter to match addresses on both the local part and the specified domain name. If no domain name is specified, the match will be on the local part only, and therefore all domain names will be accepted.

The `--address` option may be specified multiples times and can also be combined with `--address-file`.

In an address file, empty lines and lines starting with the `#` character are ignored.

The private key's length must be either 128 or 256 bits. To generate a 128 bits key, the following command is recommended:

```
openssl rand -base64 16
```

Example configuration:

```
# Sub-addresses
smtp sub-addr-delim "+"
filter "sake" proc-exec "filter-sake -s '+' -a 'a@example.org:11voiefK5PgCX5F1TTcuoQ==' -a 'b:3pUdigGQNXYBeKJdYDdERQ=='"

# Tables
table domains { "example.org", "example.com" }
table vusers { "test" = "1000:100:/var/vmail/test", "b" = "1000:100:/var/vmail/b" }
table aliases { "a" = "test" }

# Listening
listen on 127.0.0.1 hostname localhost filter "sake"
listen on ::1 hostname localhost filter "sake"

# Delivering
action "deliver" maildir userbase <vusers> alias <aliases>
match from any for domain <domains> action "deliver"
```


## Code generation protocol

Let start with some definition. For this protocol, an email address is composed of a local part, a sub-address delimiter, a sub-address, another sub-address delimiter, the validation code, the at sign and the domain name. For instance, for the address `darra.service.gizti5lj@mail.example.org`:
- local part: `darra`
- sub-address delimiter: `.`
- sub-address: `service`
- validation code: `gizti5lj`
- domain name: `mail.example.org`

The code generation protocol is based on the HMAC-SHA-256 function. The hasher is configured with the private key, then the following data is hashed, in this order: the local part, the sub-address delimiter and the sub-address.

This hash is then reduced to 5 bytes using the following dynamic offset truncation method. From the last byte of the hash, we take the last 4 bits, which gives an offset between 0 and 15. We then take the 5 bytes of the hash located at this offset.

The code is then generated by encoding those 5 bytes using base32 ([RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648)) without padding.


## Frequently Asked Questions

### How do I generate valid addresses?

The filter itself is useful for OpenSMTPD only, it is not meant to be used directly by the user. For this usage, you should use [sake-app](https://github.com/breard-r/sake-app).

In the event you do not wish to host it yourself, you can use [https://sake.email/](https://sake.email/). This is a client-side only application, your data will be exclusively stored in your browser's local storage.

### Do I need to have several mailboxes?

No.

The local part can either be a real mailbox or an alias. It is up to you to decide how to setup you mail server.

### Does it works with Postfix / Exim / whatever?

No, this project is based on the filter API used by OpenSMTPD.

### Does it supports IDN?

Yes, internationalized domain names (IDN) are supported. You can specify domain names either using valid UTF-8 or Punycode ([RFC 3492](https://datatracker.ietf.org/doc/html/rfc3492)).

### How long should be my private key?

Privates keys must have a length of either 128 bits (16 bytes) or 256 bits (32 bytes). Unless you have some very specific needs, you should choose a 128 bits key.

### What about key rotation?

Rotating the key would mean that all previously generated addresses for this local part would suddenly be invalid. Therefore, the key associated with a local part must not change.

That said, you can add a new local part that uses a new key and stop using the previous one. To this end, it is recommended to use discardable names. Local parts composed of one to three characters without special meaning are good candidates to this.

### Is the code cryptographically secure?

No, it is not.

Efforts have been made so it is almost impossible to use one or several known valid addresses to create new addresses or recover the key. However, it may not be considered cryptographically secure because of code's short length (5 bytes).

### Can you detail the efforts made to get a mostly secure code?

At the time of writing, the HMAC-SHA-256 function has no known vulnerability.

The dynamic offset truncation is a simplified version of the one defined in [section 5.3 of RFC 4226](https://datatracker.ietf.org/doc/html/rfc4226#section-5.3) (HOTP: An HMAC-Based One-Time Password Algorithm). However, reducing the 32 bytes output into 5 bytes can never be considered completely secure.

Therefore, although it cannot be considered cryptographically secure, efforts have been made to generate a code that is sufficiently resistant to most attackers.

If your threat-model includes attackers that are backed by a government or a powerful criminal organization, you should seek for professional help instead of trusting random projects on the internet.
