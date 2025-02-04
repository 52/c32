`c32`
===============

[![Crates.io](https://img.shields.io/crates/v/c32.svg)][Crates.io]
[![Documentation](https://docs.rs/c32/badge.svg)][Docs.rs]
[![Build Status](https://img.shields.io/github/actions/workflow/status/52/c32/rust.yml?branch=master)][Workflow]
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)][License-Apache]
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)][License-MIT]

Rust implementation of [Crockford's Base32][Crockford] encoding scheme.

```rust
use c32::encode;
use c32::decode;

let data = b"semper prorsum";
let encoded = encode(data);
let decoded = decode(&encoded).unwrap();

assert_eq!(encoded, "1SPAVBGCNS20W3JDXS76XBD");
assert_eq!(&decoded, data);
```

### License

<sup>
Licensed under <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your discretion.
</sup>

<br>

<sup>
Contributions to this crate will be dual-licensed under Apache-2.0 and MIT by default, unless specifically indicated otherwise.
</sup>

[Crates.io]: https://crates.io/crates/c32
[Docs.rs]: https://docs.rs/c32
[Workflow]: https://github.com/52/c32/actions
[License-Apache]: https://opensource.org/licenses/Apache-2.0
[License-MIT]: https://opensource.org/licenses/MIT
[Crockford]: https://www.crockford.com/base32.html
