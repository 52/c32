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
c32 = "0.4.0"
```

## Implementation

* **Lightweight** — The core functionality has zero external dependencies.
* **Portable** — Fully compatible with `#![no_std]` environments.
* **Safe** — Implemented entirely in safe Rust with no `unsafe` blocks.

## Examples

```rust
let bytes = b"usque ad finem";
let encoded = c32::encode(&bytes);
assert_eq!(encoded, "1TQ6WBNCMG62S10CSMPWSBD");
```

```rust
let bytes = b"usque ad finem";
let decoded = c32::decode("1TQ6WBNCMG62S10CSMPWSBD")?;
assert_eq!(decoded, bytes);
```

### In `#![no_std]` Environments

For environments without allocation support, the library provides buffer-based APIs:

```rust
// encoding with a pre-allocated buffer
let bytes = b"usque ad finem";
let mut buffer = [0; 32];

let written = c32::encode_into(bytes, &mut buffer)?;
let encoded = &buffer[..written];
assert_eq!(encoded, b"1TQ6WBNCMG62S10CSMPWSBD");
```

```rust
// decoding with a pre-allocated buffer
let encoded = b"1TQ6WBNCMG62S10CSMPWSBD";
let mut buffer = [0; 32];

let written = c32::decode_into(encoded, &mut buffer)?;
let decoded = &buffer[..written];
assert_eq!(decoded, b"usque ad finem");
```

### Using the `Buffer` Type (Const-Friendly)

The library provides a [`Buffer`] type that can be used in const contexts:

```rust
use c32::Buffer;

const INPUT: [u8; 3] = [42, 42, 42];
const ENCODED: Buffer<5> = Buffer::encode(&INPUT);
assert_eq!(ENCODED.as_str(), "2MAHA");

const DECODED: Buffer<5> = Buffer::decode(b"2MAHA");
assert_eq!(DECODED.as_bytes(), [42, 42, 42]);
```

### Checksums (`#[feature = "check"]`)

The `check` feature provides methods for encoding data with SHA256-based checksum verification.

The encoded data follows this layout:

```text
[version (1B)] + [payload (nB)] + [checksum (4B)]
```

And is computed by...

```text
1. Concatenating the version byte with the payload bytes.
2. Taking the SHA256 hash of the concatenated bytes.
3. Taking the SHA256 hash of the result.
4. Using the first 4 bytes as the checksum.
```

```rust
let bytes = b"usque ad finem";
let encoded = c32::encode_check(bytes, 22)?;
assert_eq!(encoded, "P7AWVHENJJ0RB441K6JVK5DNJ7J3V5");
```

```rust
let encoded = "P7AWVHENJJ0RB441K6JVK5DNJ7J3V5";
let (decoded, version) = c32::decode_check(encoded)?;
assert_eq!(decoded, b"usque ad finem");
assert_eq!(version, 22);
```

For more details, please refer to the full [API Reference][Docs.rs].

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
