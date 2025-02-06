`c32`
===============

[![Crates.io](https://img.shields.io/crates/v/c32.svg)][Crates.io]
[![Documentation](https://docs.rs/c32/badge.svg)][Docs.rs]
[![Build Status](https://img.shields.io/github/actions/workflow/status/52/c32/rust.yml?branch=master)][Workflow]
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)][License-Apache]
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)][License-MIT]

Rust implementation of [Crockford's Base32][Crockford] encoding scheme.

```toml
[dependencies]
c32 = "0.3.0"
```

## Usage

This crate provides two approaches for encoding/decoding:

```rust
// with `alloc` ...

#[cfg(feature = "alloc")]
fn encode() {
    let bytes = b"usque ad finem";
    let encoded = c32::encode(bytes);
    assert_eq!(encoded, "1TQ6WBNCMG62S10CSMPWSBD");
}

#[cfg(feature = "alloc")]
fn decode() {
    let encoded = "1TQ6WBNCMG62S10CSMPWSBD";
    let decoded = c32::decode(encoded).unwrap();
    assert_eq!(decoded, b"usque ad finem");
}

// or 'no_std' ...

fn encode_no_std() {
    const bytes: &[u8; 14] = b"usque ad finem";
    let mut buffer = [0; c32::encoded_len(bytes.len())];
    let pos = c32::encode_into(bytes, &mut buffer).unwrap();
    assert_eq!(&buffer[..pos], b"1TQ6WBNCMG62S10CSMPWSBD")
}

fn decode_no_std() {
    const bytes: &[u8; 23] = b"1TQ6WBNCMG62S10CSMPWSBD";
    let mut buffer = [0; c32::decoded_len(bytes.len())];
    let pos = c32::decode_into(bytes, &mut buffer).unwrap();
    assert_eq!(&buffer[..pos], b"usque ad finem")
}
```

## Security

<sup>
For security-related concerns, please review the <a href="SECURITY.md">Security Policy</a>.
</sup>

## License

<sup>
Licensed under <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT License</a> at your discretion.
</sup>

## Contribution

<sup>
Contributions to this crate will be dual-licensed under <a href="LICENSE-APACHE">Apache-2.0</a> and <a href="LICENSE-MIT">MIT</a> by default, unless specifically indicated otherwise.
</sup>

[Crates.io]: https://crates.io/crates/c32
[Docs.rs]: https://docs.rs/c32
[Workflow]: https://github.com/52/c32/actions
[License-Apache]: https://opensource.org/licenses/Apache-2.0
[License-MIT]: https://opensource.org/licenses/MIT
[Crockford]: https://www.crockford.com/base32.html
