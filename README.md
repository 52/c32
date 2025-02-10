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

## Implementation

* **Lightweight** — The core functionality has zero external dependencies.
* **Portable** — Fully compatible with `#![no_std]` environments.
* **Safe** — Implemented entirely in safe Rust with no `unsafe` blocks.

## Examples

#### `std` or `alloc`

```rust
// encoding...
let bytes = b"usque ad finem";
let encoded = c32::encode(&bytes);
assert_eq!(encoded, "1TQ6WBNCMG62S10CSMPWSBD");
```

```rust
// decoding...
let bytes = b"usque ad finem";
let decoded = c32::decode("1TQ6WBNCMG62S10CSMPWSBD")?;
assert_eq!(decoded, bytes);
```

#### `#![no_std]`

```rust
// encoding...
let bytes = b"usque ad finem";
let mut buffer = [0; 32];

let written = c32::encode_into(bytes, &mut buffer)?;
let encoded = &buffer[..written];
assert_eq!(encoded, b"1TQ6WBNCMG62S10CSMPWSBD");
```

```rust
// decoding...
let encoded = b"1TQ6WBNCMG62S10CSMPWSBD";
let mut buffer = [0; 32];

let written = c32::decode_into(encoded, &mut buffer)?;
let decoded = &buffer[..written];
assert_eq!(decoded, b"usque ad finem");
```

## Checksum

The `check` feature provides methods for encoding data with SHA256-based checksum verification.

#### `std` or `alloc`

```rust
// encoding...
let bytes = b"usque ad finem";
let encoded = c32::encode_check(bytes, 22)?;
assert_eq!(encoded, "P7AWVHENJJ0RB441K6JVK5DNJ7J3V5");
```

```rust
// decoding...
let encoded = "P7AWVHENJJ0RB441K6JVK5DNJ7J3V5";
let (version, decoded) = c32::decode_check(encoded)?;
assert_eq!(decoded, b"usque ad finem");
assert_eq!(version, 22);
```

#### `#![no_std]`

```rust
// encoding...
let bytes = b"usque ad finem";
let mut buffer = [0; 32];

let written = c32::encode_check_into(bytes, 22, &mut buffer)?;
let encoded = &buffer[..written];
assert_eq!(encoded, b"P7AWVHENJJ0RB441K6JVK5DNJ7J3V5");
```

```rust
// decoding...
let encoded = b"P7AWVHENJJ0RB441K6JVK5DNJ7J3V5";
let mut buffer = [0; 32];

let (version, written) = c32::decode_check_into(encoded, &mut buffer)?;
let decoded = &buffer[..written];
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
