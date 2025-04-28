// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#![no_std]
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::items_after_statements)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! [![Crates.io](https://img.shields.io/crates/v/c32.svg)][Crates.io]
//! [![Documentation](https://docs.rs/c32/badge.svg)][Docs.rs]
//! [![Build Status](https://img.shields.io/github/actions/workflow/status/52/c32/rust.yml?branch=master)][Workflow]
//! [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)][License-Apache]
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)][License-MIT]
//!
//! Rust implementation of [Crockford's Base32][Crockford] encoding scheme.
//!
//! ## Implementation
//!
//! * **Lightweight** — The core functionality has zero external dependencies.
//! * **Portable** — Fully compatible with `#![no_std]` environments.
//! * **Safe** — Implemented entirely in safe Rust with no `unsafe` blocks.
//!
//! ```rust
//! # #[cfg(feature = "alloc")] {
//! let bytes = b"usque ad finem";
//! let encoded = c32::encode(&bytes);
//! assert_eq!(encoded, "1TQ6WBNCMG62S10CSMPWSBD");
//! # }
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! ```rust
//! # #[cfg(feature = "alloc")] {
//! let bytes = b"usque ad finem";
//! let decoded = c32::decode("1TQ6WBNCMG62S10CSMPWSBD")?;
//! assert_eq!(decoded, bytes);
//! # }
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! ## In `#![no_std]` Environments
//!
//! For environments without allocation support, the library provides buffer-based APIs:
//!
//! ```rust
//! // encoding with a pre-allocated buffer
//! let bytes = b"usque ad finem";
//! let mut buffer = [0; 32];
//!
//! let written = c32::encode_into(bytes, &mut buffer)?;
//! let encoded = &buffer[..written];
//! assert_eq!(encoded, b"1TQ6WBNCMG62S10CSMPWSBD");
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! ```rust
//! // decoding with a pre-allocated buffer
//! let encoded = b"1TQ6WBNCMG62S10CSMPWSBD";
//! let mut buffer = [0; 32];
//!
//! let written = c32::decode_into(encoded, &mut buffer)?;
//! let decoded = &buffer[..written];
//! assert_eq!(decoded, b"usque ad finem");
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! ## Using the `Buffer` Type (Const-Friendly)
//!
//! The library provides a [`Buffer`] type that can be used in const contexts:
//!
//! ```rust
//! use c32::Buffer;
//!
//! const INPUT: [u8; 3] = [42, 42, 42];
//! const ENCODED: Buffer<5> = Buffer::encode(&INPUT);
//! assert_eq!(ENCODED.as_str(), "2MAHA");
//!
//! const DECODED: Buffer<5> = Buffer::decode(b"2MAHA");
//! assert_eq!(DECODED.as_bytes(), [42, 42, 42]);
//! ```
//!
//! ## Checksums (`#[feature = "check"]`)
//!
//! The `check` feature enables methods for encoding data with SHA256-based checksum verification.
//!
//! The encoded data follows this layout:
//!
//! ```text
//! [version (1B)] + [payload (nB)] + [checksum (4B)]
//! ```
//!
//! And is computed by...
//!
//! ```text
//! 1. Concatenating the version byte with the payload bytes.
//! 2. Taking the SHA256 hash of the concatenated bytes.
//! 3. Taking the SHA256 hash of the result.
//! 4. Using the first 4 bytes as the checksum.
//! ```
//!
//! ### Examples
//!
//! ```rust
//! # #[cfg(all(feature = "check", feature = "alloc"))] {
//! let bytes = b"usque ad finem";
//! let encoded = c32::encode_check(bytes, 22)?;
//! assert_eq!(encoded, "P7AWVHENJJ0RB441K6JVK5DNJ7J3V5");
//! # }
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! ```rust
//! # #[cfg(all(feature = "check", feature = "alloc"))] {
//! let encoded = "P7AWVHENJJ0RB441K6JVK5DNJ7J3V5";
//! let (decoded, version) = c32::decode_check(encoded)?;
//! assert_eq!(decoded, b"usque ad finem");
//! assert_eq!(version, 22);
//! # }
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! # Features
//!
//!  Feature | Description
//! ---------|-------------------------------------------------------------
//!  `alloc` | Allocation-based API via [`encode`] and [`decode`]
//!  `check` | Support for checksum validation
//!
//! For more details, please refer to the full [API Reference][Docs.rs].
//!
//! [Crates.io]: https://crates.io/crates/c32
//! [Docs.rs]: https://docs.rs/c32
//! [Workflow]: https://github.com/52/c32/actions
//! [License-Apache]: https://opensource.org/licenses/Apache-2.0
//! [License-MIT]: https://opensource.org/licenses/MIT
//! [Crockford]: https://www.crockford.com/base32.html

#[cfg(feature = "alloc")]
extern crate alloc;

use core::error;
use core::fmt;
use core::slice;
use core::str;

/// Re-exports for feature compatibility.
///
/// This module exports common allocation types.
#[cfg(feature = "alloc")]
pub(crate) mod __private {
    pub use alloc::string::String;
    pub use alloc::vec;
    pub use alloc::vec::Vec;
}

#[cfg(feature = "alloc")]
pub(crate) use __private::*;

/// This module provides methods for computing [`SHA-256`] checksums.
///
/// [`SHA-256`]: https://helix.stormhub.org/papers/SHA-256.pdf
#[cfg(feature = "check")]
pub mod checksum {
    use sha2::Sha256;

    use super::*;

    /// Length of the checksum in bytes.
    pub const BYTE_LENGTH: usize = 4;

    /// A type alias for a [`Sha256`] checksum.
    pub type Checksum = [u8; BYTE_LENGTH];

    /// Computes a 4-byte [`Checksum`]from a byte array and version.
    ///
    /// # Notes
    ///
    /// The checksum is computed by:
    ///
    /// 1. Concatenating the version byte with the payload bytes.
    /// 2. Taking the SHA256 hash of the concatenated bytes.
    /// 3. Taking the SHA256 hash of the result.
    /// 4. Using the first 4 bytes as the checksum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::checksum;
    /// let bytes = [42, 42, 42];
    /// let sum = checksum::compute(&bytes, 0);
    /// assert_eq!(sum.len(), 4);
    /// ```
    #[inline]
    #[must_use]
    pub const fn compute(bytes: &[u8], version: u8) -> Checksum {
        let buffer = Sha256::new().update(&[version]).update(bytes).finalize();
        let hash = Sha256::new().update(&buffer).finalize();
        from_slice(&hash)
    }

    /// Creates a [`Checksum`] from a byte slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::checksum;
    /// let hash = [1, 2, 3, 4, 5, 6, 7, 8];
    /// let sum = checksum::from_slice(&hash);
    /// assert_eq!(sum, [1, 2, 3, 4]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_slice(bytes: &[u8]) -> Checksum {
        let mut sum = [0u8; BYTE_LENGTH];
        __internal::memcpy(&mut sum, 0, bytes, 0, BYTE_LENGTH);
        sum
    }
}

/// The Crockford Base32 alphabet used for encoding and decoding.
pub(crate) const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// A mapping from ASCII characters to their Crockford Base32 values.
pub(crate) const BYTE_MAP: [i8; 128] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, 16, 17, 1,
    18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, -1, 10,
    11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30,
    31, -1, -1, -1, -1, -1,
];

/// Error variants for fallible Crockford Base32 operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// The buffer size is insufficient for the operation.
    BufferTooSmall {
        /// The minimum required buffer size in bytes.
        min: usize,
        /// The actual size of the provided buffer in bytes.
        len: usize,
    },
    /// An invalid character was encountered during decoding.
    InvalidCharacter {
        /// The invalid character found in the input.
        char: char,
        /// The byte index of the character.
        index: usize,
    },
    /// The expected prefix character is missing.
    MissingPrefix {
        /// The expected prefix character.
        char: char,
        /// The actual first character found (or None if the string is empty).
        got: Option<char>,
    },
    #[cfg(feature = "check")]
    /// The provided version byte is invalid.
    InvalidVersion {
        /// A description of the expected version constraints.
        expected: &'static str,
        /// The invalid version byte.
        version: u8,
    },
    /// The input has fewer bytes than are required.
    #[cfg(feature = "check")]
    InsufficientData {
        /// The minimum required amount of bytes.
        min: usize,
        /// The actual number of bytes provided.
        len: usize,
    },
    /// The computed checksum does not match the expected sum.
    #[cfg(feature = "check")]
    ChecksumMismatch {
        /// The expected checksum.
        expected: checksum::Checksum,
        /// The actual checksum.
        got: checksum::Checksum,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferTooSmall { min, len } => {
                write!(f, "Buffer size '{len}' is less than required '{min}'")
            }
            Self::InvalidCharacter { char, index } => {
                write!(f, "Invalid character '{char}' at position {index}")
            }
            Self::MissingPrefix { char, got } => {
                write!(f, "Expected prefix '{char}', found '{got:?}'")
            }
            #[cfg(feature = "check")]
            Self::InvalidVersion { expected, version } => {
                write!(f, "Invalid version byte '{version}': {expected}")
            }
            #[cfg(feature = "check")]
            Self::InsufficientData { min, len } => {
                write!(f, "Input size '{len}' is less than required '{min}'")
            }
            #[cfg(feature = "check")]
            Self::ChecksumMismatch { expected, got } => {
                write!(f, "Expected checksum '{expected:?}', got '{got:?}'")
            }
        }
    }
}

impl error::Error for Error {}

/// Result type for fallible Crockford Base32 operations.
pub type Result<T> = core::result::Result<T, Error>;

/// A fixed-size buffer for encoding or decoding Crockford's Base32.
///
/// [`Buffer`] manages a fixed-size array of bytes and tracks the number of
/// bytes written during encoding and decoding operations, and is primarily
/// designed for use in a constant context.
///
/// # Examples
///
/// ```rust
/// # use c32::Buffer;
/// const INPUT: [u8; 3] = [42, 42, 42];
/// const ENC: Buffer<5> = Buffer::encode(&INPUT);
/// assert_eq!(ENC.as_str(), "2MAHA");
/// assert_eq!(ENC.pos(), 5);
/// ```
///
/// ```rust
/// # use c32::Buffer;
/// const INPUT: [u8; 5] = *b"2MAHA";
/// const DEC: Buffer<5> = Buffer::decode(&INPUT);
/// assert_eq!(DEC.as_bytes(), [42, 42, 42]);
/// assert_eq!(DEC.pos(), 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Buffer<const N: usize> {
    /// The underlying byte array.
    dst: [u8; N],
    /// The number of written bytes to the `dst` array.
    dst_pos: usize,
}

impl<const N: usize> Buffer<N> {
    /// An empty [`Buffer`] with no bytes written.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const BUFFER: Buffer<1> = Buffer::<1>::EMPTY;
    /// assert_eq!(BUFFER.as_bytes(), &[]);
    /// assert_eq!(BUFFER.pos(), 0);
    /// ```
    pub const EMPTY: Self = Buffer {
        dst: [0u8; N],
        dst_pos: 0,
    };

    /// Encodes a byte array into a [`Buffer`].
    ///
    /// # Panics
    ///
    /// This method panics if:
    ///
    /// - The buffer size `N` is `N >= encoded_len(M)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const INPUT: [u8; 3] = [42, 42, 42];
    /// const ENC: Buffer<5> = Buffer::encode(&INPUT);
    /// assert_eq!(ENC.as_str(), "2MAHA");
    /// ```
    #[inline]
    #[must_use]
    pub const fn encode<const M: usize>(src: &[u8; M]) -> Buffer<N> {
        assert!(N >= encoded_len(M), "Size 'N' is too small");

        // Allocate the output buffer.
        let mut dst = [0u8; N];

        // Encode the input to the buffer.
        let dst_pos = __internal::en(src, 0, M, &mut dst, 0, None);

        Buffer { dst, dst_pos }
    }

    /// Decodes a slice of encoded bytes into a [`Buffer`].
    ///
    /// # Panics
    ///
    /// This method panics if:
    ///
    /// - The buffer size `N` is `N >= decoded_len(src.len())`.
    /// - The input contains invalid Crockford Base32 characters.
    /// - The input contains non-ASCII bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const INPUT: [u8; 5] = *b"2MAHA";
    /// const DEC: Buffer<5> = Buffer::decode(&INPUT);
    /// assert_eq!(DEC.as_bytes(), [42, 42, 42]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn decode(src: &[u8]) -> Buffer<N> {
        assert!(N >= decoded_len(src.len()), "Size 'N' is too small");

        // Allocate the output buffer.
        let mut dst = [0u8; N];

        // Decode the input to the buffer.
        let dst_pos = match __internal::de(src, 0, src.len(), &mut dst, 0) {
            Ok(pos) => pos,
            Err(Error::InvalidCharacter { char: _, index: _ }) => {
                panic!("Input contains invalid characters")
            }
            _ => unreachable!(),
        };

        Buffer { dst, dst_pos }
    }

    /// Encodes a byte array with a prefix into a [`Buffer`].
    ///
    /// # Panics
    ///
    /// This method panics if:
    ///
    /// - The buffer size `N` is `N >= encoded_len(M)`.
    /// - The prefix is a non-ASCII character.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const INPUT: [u8; 3] = [42, 42, 42];
    /// const ENC: Buffer<6> = Buffer::encode_prefixed(&INPUT, 'S');
    /// assert_eq!(ENC.as_str(), "S2MAHA");
    /// ```
    #[inline]
    #[must_use]
    pub const fn encode_prefixed<const M: usize>(src: &[u8; M], prefix: char) -> Buffer<N> {
        assert!(N > encoded_len(M), "Size 'N' is too small");
        assert!(prefix.is_ascii(), "Prefix must be an ASCII character");

        // Allocate the output buffer.
        let mut dst = [0u8; N];

        // Prepend the prefix character.
        dst[0] = prefix as u8;

        // Encode the input to the buffer.
        let dst_pos = __internal::en(src, 0, M, &mut dst, 1, None) + 1;

        Buffer { dst, dst_pos }
    }

    /// Decodes a slice of prefixed encoded bytes into a [`Buffer`].
    ///
    /// # Panics
    ///
    /// This method panics if:
    ///
    /// - The buffer size `N` is `N >= decoded_len(src.len() - 1)`.
    /// - The input contains invalid Crockford Base32 characters.
    /// - The input does not start with the expected prefix.
    /// - The input contains non-ASCII bytes.
    /// - The prefix is a non-ASCII character.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const INPUT: [u8; 6] = *b"S2MAHA";
    /// const DEC: Buffer<6> = Buffer::decode_prefixed(&INPUT, 'S');
    /// assert_eq!(DEC.as_bytes(), [42, 42, 42]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn decode_prefixed(src: &[u8], prefix: char) -> Buffer<N> {
        assert!(N >= decoded_len(src.len() - 1), "Size 'N' is too small");
        assert!(prefix.is_ascii(), "Prefix must be an ASCII character");
        assert!(src[0] == prefix as u8, "Input must start with prefix");

        // Allocate the output buffer.
        let mut dst = [0u8; N];

        // Decode the input (without prefix) to the buffer.
        let dst_pos = match __internal::de(src, 1, src.len() - 1, &mut dst, 0) {
            Ok(pos) => pos,
            Err(Error::InvalidCharacter { char: _, index: _ }) => {
                panic!("Input contains invalid characters")
            }
            _ => unreachable!(),
        };

        Buffer { dst, dst_pos }
    }

    /// Encodes a byte array with a checksum into a [`Buffer`].
    ///
    /// # Panics
    ///
    /// This method panics if:
    ///
    /// - The buffer size `N` is `N >= encoded_check_len(M)`.
    /// - The version is greater than or equal to 32.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const INPUT: [u8; 3] = [42, 42, 42];
    /// const ENC: Buffer<13> = Buffer::encode_check(&INPUT, 0);
    /// assert_eq!(ENC.as_str(), "0AHA59B9201Z");
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "check")]
    pub const fn encode_check<const M: usize>(src: &[u8; M], version: u8) -> Buffer<N> {
        assert!(N >= encoded_check_len(M), "Size 'N' is too small");
        assert!(version < 32, "Version must be < 32");

        // Allocate the output buffer.
        let mut dst = [0u8; N];

        // Prepend the version character.
        dst[0] = ALPHABET[version as usize];

        // Compute the checksum.
        let sum = checksum::compute(src, version);

        // Encode the input and checksum to the buffer.
        let dst_pos = __internal::en(src, 0, M, &mut dst, 1, Some(sum)) + 1;

        Buffer { dst, dst_pos }
    }

    /// Decodes a slice of check-encoded bytes into a [`Buffer`].
    ///
    /// # Panics
    ///
    /// This method panics if:
    ///
    /// - The buffer size `N` is insufficient for the decoded output.
    /// - The input is shorter than 2 bytes.
    /// - The input contains invalid Crockford Base32 characters.
    /// - The input contains non-ASCII bytes.
    /// - The version is greater than or equal to 32.
    /// - The computed checksum does not match the embedded checksum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const INPUT: [u8; 12] = *b"0AHA59B9201Z";
    /// const DEC: Buffer<12> = Buffer::decode_check(&INPUT);
    /// assert_eq!(DEC.as_bytes(), [42, 42, 42]);
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "check")]
    pub const fn decode_check(src: &[u8]) -> Buffer<N> {
        assert!(N >= decoded_check_len(src.len()), "Size 'N' is too small");
        assert!(src.len() >= 2, "Input must be at least 2 bytes");

        // Allocate the output buffer.
        let mut dst = [0u8; N];

        // Extract the version byte.
        let mut buffer = [0u8; 1];
        let _ = match __internal::de(&[src[0]], 0, 1, &mut buffer, 0) {
            Ok(pos) => pos,
            Err(Error::InvalidCharacter { char: _, index: _ }) => {
                panic!("Input contains invalid characters")
            }
            _ => unreachable!(),
        };

        // Assert that the version is < 32.
        let version = buffer[0];
        assert!(version < 32, "Version must be < 32");

        // Decode the payload.
        let offset = match __internal::de(src, 1, src.len() - 1, &mut dst, 0) {
            Ok(pos) => pos,
            Err(Error::InvalidCharacter { char: _, index: _ }) => {
                panic!("Input contains invalid characters")
            }
            _ => unreachable!(),
        };

        let dst_pos = offset - checksum::BYTE_LENGTH;

        // Extract the checksum.
        let mut sum = [0u8; checksum::BYTE_LENGTH];
        __internal::memcpy(&mut sum, 0, &dst, dst_pos, checksum::BYTE_LENGTH);

        // Compute the expected checksum.
        let mut expected = [0u8; checksum::BYTE_LENGTH];
        __internal::memcpy(&mut expected, 0, &dst, dst_pos, 4);

        // Assert that the computed and actual checksums match.
        assert!(__internal::memcmp(&expected, &sum, 4), "Checksum mismatch");

        Buffer { dst, dst_pos }
    }

    /// Encodes a byte array with a checksum and prefix into a [`Buffer`].
    ///
    /// # Panics
    ///
    /// This method panics if:
    ///
    /// - The buffer size `N` is insufficient for the encoded output and prefix.
    /// - The prefix is not an ASCII character.
    /// - The version is greater than or equal to 32.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const INPUT: [u8; 3] = [42, 42, 42];
    /// const ENC: Buffer<14> = Buffer::encode_check_prefixed(&INPUT, 'S', 0);
    /// assert_eq!(ENC.as_str(), "S0AHA59B9201Z");
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "check")]
    pub const fn encode_check_prefixed<const M: usize>(
        src: &[u8; M],
        prefix: char,
        version: u8,
    ) -> Buffer<N> {
        assert!(N > encoded_check_len(M), "Size 'N' is too small");
        assert!(prefix.is_ascii(), "Prefix must be an ASCII character");
        assert!(version < 32, "Version must be < 32");

        // Allocate the output buffer.
        let mut dst = [0u8; N];

        // Prepend the prefix character.
        dst[0] = prefix as u8;

        // Prepend the version character.
        dst[1] = ALPHABET[version as usize];

        // Compute the checksum.
        let sum = checksum::compute(src, version);

        // Encode the input and checksum to the buffer.
        let dst_pos = __internal::en(src, 0, M, &mut dst, 2, Some(sum)) + 2;

        Buffer { dst, dst_pos }
    }

    /// Decodes a slice of prefixed check-encoded bytes into a [`Buffer`].
    ///
    /// # Panics
    ///
    /// This method panics if:
    ///
    /// - The buffer size `N` is insufficient for the decoded output.
    /// - The input is shorter than 3 bytes.
    /// - The prefix is not an ASCII character.
    /// - The input does not start with the expected prefix.
    /// - The input contains invalid Crockford Base32 characters.
    /// - The input contains non-ASCII bytes.
    /// - The version version is greater than or equal to 32.
    /// - The computed checksum does not match the embedded checksum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const INPUT: [u8; 13] = *b"S0AHA59B9201Z";
    /// const DEC: Buffer<13> = Buffer::decode_check_prefixed(&INPUT, 'S');
    /// assert_eq!(DEC.as_bytes(), [42, 42, 42]);
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "check")]
    pub const fn decode_check_prefixed(src: &[u8], prefix: char) -> Buffer<N> {
        assert!(N >= decoded_check_len(src.len() - 1), "'N' is too small");
        assert!(src.len() >= 3, "Input must be at least 3 bytes");
        assert!(prefix.is_ascii(), "Prefix must be an ASCII character");
        assert!(src[0] == prefix as u8, "Input must start with prefix");

        // Extract the version byte.
        let mut buffer = [0u8; 1];
        let _ = match __internal::de(&[src[1]], 0, 1, &mut buffer, 0) {
            Ok(pos) => pos,
            Err(Error::InvalidCharacter { char: _, index: _ }) => {
                panic!("Input contains invalid characters")
            }
            _ => unreachable!(),
        };

        // Assert that the version is < 32.
        let version = buffer[0];
        assert!(version < 32, "Version must be < 32");

        // Allocate the output buffer.
        let mut dst = [0u8; N];

        // Decode the payload.
        let offset = match __internal::de(src, 2, src.len() - 2, &mut dst, 0) {
            Ok(pos) => pos,
            Err(Error::InvalidCharacter { char: _, index: _ }) => {
                panic!("Input contains invalid characters")
            }
            _ => unreachable!(),
        };

        let dst_pos = offset - checksum::BYTE_LENGTH;

        // Extract the checksum.
        let mut sum = [0u8; checksum::BYTE_LENGTH];
        __internal::memcpy(&mut sum, 0, &dst, dst_pos, checksum::BYTE_LENGTH);

        // Compute the expected checksum.
        let mut expected = [0u8; checksum::BYTE_LENGTH];
        __internal::memcpy(&mut expected, 0, &dst, dst_pos, 4);

        // Assert that the computed and actual checksums match.
        assert!(__internal::memcmp(&expected, &sum, 4), "Checksum mismatch");

        Buffer { dst, dst_pos }
    }

    /// Returns the number of bytes written to the buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const INPUT: [u8; 3] = [42, 42, 42];
    /// const ENC: Buffer<5> = Buffer::encode(&INPUT);
    /// assert_eq!(ENC.pos(), 5);
    /// ```
    #[inline]
    #[must_use]
    pub const fn pos(&self) -> usize {
        self.dst_pos
    }

    /// Returns a string slice of the written bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const INPUT: [u8; 3] = [42, 42, 42];
    /// const ENC: Buffer<5> = Buffer::encode(&INPUT);
    /// assert_eq!(ENC.as_str(), "2MAHA");
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &str {
        // SAFETY: We only push valid UTF-8 to `dst`.
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Returns a slice of the written bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// const INPUT: [u8; 3] = [42, 42, 42];
    /// const ENC: Buffer<5> = Buffer::encode(&INPUT);
    /// assert_eq!(ENC.as_bytes(), b"2MAHA");
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        // SAFETY: `dst_pos` is always within bounds.
        unsafe { slice::from_raw_parts(self.as_ptr(), self.dst_pos) }
    }

    /// Returns a raw pointer to the buffer's data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use c32::Buffer;
    /// let buffer = Buffer::<10>::EMPTY;
    /// let ptr = buffer.as_ptr();
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_ptr(&self) -> *const u8 {
        self.dst.as_ptr()
    }
}

/// Computes the required capacity for encoding into Crockford Base32.
///
/// # Notes
///
/// The calculation breaks down into:
///
/// - For every 5 bytes (40 bits), exactly 8 Base32 characters are needed.
/// - For remaining bytes, more characters are needed based on the total bits.
///
/// # Examples
///
/// ```rust
/// assert_eq!(c32::encoded_len(0), 0);
/// assert_eq!(c32::encoded_len(1), 2);
/// assert_eq!(c32::encoded_len(3), 5);
/// ```
#[inline]
#[must_use]
pub const fn encoded_len(n: usize) -> usize {
    (n * 8 + 4) / 5
}

/// Computes the required capacity for encoding into Crockford Base32Check.
///
/// # Notes
///
/// The calculation breaks down into:
///
/// - One byte for the version character.
/// - The encoded length of the payload plus a 4-byte checksum.
///
/// # Examples
///
/// ```rust
/// assert_eq!(c32::encoded_check_len(0), 8);
/// assert_eq!(c32::encoded_check_len(1), 9);
/// assert_eq!(c32::encoded_check_len(3), 13);
/// ```
#[inline]
#[must_use]
#[cfg(feature = "check")]
pub const fn encoded_check_len(n: usize) -> usize {
    1 + encoded_len(n + 4)
}

/// Computes the required capacity for decoding from Crockford Base32.
///
/// # Examples
///
/// ```rust
/// assert_eq!(c32::decoded_len(0), 0);
/// assert_eq!(c32::decoded_len(1), 1);
/// assert_eq!(c32::decoded_len(2), 2);
/// ```
#[inline]
#[must_use]
pub const fn decoded_len(n: usize) -> usize {
    n
}

/// Computes the required capacity for decoding from Crockford Base32Check.
///
/// # Examples
///
/// ```rust
/// assert_eq!(c32::decoded_check_len(8), 8);
/// assert_eq!(c32::decoded_check_len(9), 9);
/// assert_eq!(c32::decoded_check_len(13), 13);
/// ```
#[inline]
#[must_use]
#[cfg(feature = "check")]
pub const fn decoded_check_len(n: usize) -> usize {
    n
}

/// Encodes bytes into a Crockford Base32-encoded string.
///
/// # Panics
///
/// This method can panic in two cases:
///
/// - If encoding fails despitce sufficient buffer capacity.
/// - If the encoded output contains non-UTF8 bytes.
///
/// Both panics should never occur under normal circumstances.
///
/// # Examples
///
/// ```rust
/// let en = c32::encode([42, 42, 42]);
/// assert_eq!(en, "2MAHA");
/// ```
#[inline]
#[must_use]
#[cfg(feature = "alloc")]
pub fn encode<B>(src: B) -> String
where
    B: AsRef<[u8]>,
{
    let src = src.as_ref();

    // Allocate the output buffer.
    let capacity = encoded_len(src.len());
    let mut dst = vec![0u8; capacity];

    // This should not panic, as we allocate enough space.
    let offset = encode_into(src, &mut dst).unwrap();
    dst.truncate(offset);

    // This should not panic, as we only push valid ASCII.
    String::from_utf8(dst).unwrap()
}

/// Decodes a Crockford Base32-encoded string.
///
/// # Errors
///
/// This method will return an [`Error`] if:
///
/// - [`Error::InvalidCharacter`], the input contains invalid characters.
///
/// # Examples
///
/// ```rust
/// # use c32::Error;
/// let de = c32::decode("2MAHA")?;
/// assert_eq!(de, [42, 42, 42]);
/// # Ok::<(), Error>(())
/// ```
#[inline]
#[cfg(feature = "alloc")]
pub fn decode(str: &str) -> Result<Vec<u8>> {
    let bytes = str.as_bytes();

    // Allocate the output buffer.
    let capacity = decoded_len(bytes.len());
    let mut dst = vec![0u8; capacity];

    // Decode the input bytes into the buffer.
    let offset = decode_into(bytes, &mut dst)?;
    dst.truncate(offset);

    Ok(dst)
}

/// Encodes bytes into a prefixed Crockford Base32-encoded string.
///
/// # Examples
///
/// ```rust
/// let en = c32::encode_prefixed([42, 42, 42], 'S');
/// assert_eq!(en, "S2MAHA");
/// ```
#[inline]
#[must_use]
#[cfg(feature = "alloc")]
pub fn encode_prefixed<B>(src: B, prefix: char) -> String
where
    B: AsRef<[u8]>,
{
    let src = src.as_ref();

    // Encode the input bytes.
    let encoded = encode(src);

    // Allocate the output string.
    let capacity = prefix.len_utf8() + encoded.len();
    let mut dst = String::with_capacity(capacity);

    // Append the prefix and encoded string.
    dst.push(prefix);
    dst.push_str(&encoded);
    dst
}

/// Decodes a prefixed Crockford Base32-encoded string.
///
/// # Errors
///
/// This method will return an [`Error`] if:
///
/// - [`Error::MissingPrefix`], the input does not start with the prefix.
/// - [`Error::InvalidCharacter`], the input contains invalid characters.
///
/// # Examples
///
/// ```rust
/// # use c32::Error;
/// let de = c32::decode_prefixed("P2MAHA", 'P')?;
/// assert_eq!(de, [42, 42, 42]);
/// # Ok::<(), Error>(())
/// ```
#[inline]
#[cfg(feature = "alloc")]
pub fn decode_prefixed(str: &str, prefix: char) -> Result<Vec<u8>> {
    // Assert that the string starts with the prefix.
    if !str.starts_with(prefix) {
        return Err(Error::MissingPrefix {
            char: prefix,
            got: str.chars().next(),
        });
    }

    // Skip the prefix character and decode the rest.
    match decode(&str[prefix.len_utf8()..]) {
        Ok(bytes) => Ok(bytes),
        Err(Error::InvalidCharacter { char, index }) => {
            // This adjusts the index in an 'InvalidCharacter' to account for
            // the prefix in the original string that we don't decode.
            Err(Error::InvalidCharacter {
                char,
                index: index + prefix.len_utf8(),
            })
        }
        Err(e) => Err(e),
    }
}

/// Encodes bytes into a Crockford Base32Check-encoded string.
///
/// # Panics
///
/// This method can panic in two cases:
///
/// - If encoding fails despite sufficient buffer capacity.
/// - If the encoded output contains non-UTF8 bytes.
///
/// Both panics should never occur under normal circumstances.
///
/// # Errors
///
/// This method will return an [`Error`] if:
///
/// - [`Error::InvalidVersion`], the version is 32 or greater.
///
/// # Examples
///
/// ```rust
/// # use c32::Error;
/// let en = c32::encode_check([42, 42, 42], 0)?;
/// assert_eq!(en, "0AHA59B9201Z");
/// # Ok::<(), Error>(())
/// ```
#[inline]
#[cfg(all(feature = "alloc", feature = "check"))]
pub fn encode_check<B>(src: B, version: u8) -> Result<String>
where
    B: AsRef<[u8]>,
{
    let src = src.as_ref();

    // Allocate the output string.
    let capacity = encoded_check_len(src.len());
    let mut dst = vec![0u8; capacity];

    // This should not panic, as we allocate enough space.
    let offset = encode_check_into(src, &mut dst, version)?;
    dst.truncate(offset);

    // This should not panic, as we only push valid ASCII.
    Ok(String::from_utf8(dst).unwrap())
}

/// Decodes a Crockford Base32Check-encoded string.
///
/// # Errors
///
/// This method will return an [`Error`] if:
///
/// - [`Error::InvalidCharacter`], the input contains invalid characters.
/// - [`Error::InsufficientData`], the input has fewer bytes than required.
/// - [`Error::ChecksumMismatch`], the checksum's do not match.
/// - [`Error::InvalidVersion`], the version is 32 or greater.
///
/// # Examples
///
/// ```rust
/// # use c32::Error;
/// let (bytes, version) = c32::decode_check("0AHA59B9201Z")?;
/// assert_eq!(bytes, [42, 42, 42]);
/// assert_eq!(version, 0);
/// # Ok::<(), Error>(())
/// ```
#[inline]
#[cfg(all(feature = "alloc", feature = "check"))]
pub fn decode_check(str: &str) -> Result<(Vec<u8>, u8)> {
    let bytes = str.as_bytes();

    // Allocate the output buffer.
    let capacity = decoded_check_len(bytes.len());
    let mut dst = vec![0u8; capacity];

    // Decode the input bytes into the buffer.
    let (offset, version) = decode_check_into(bytes, &mut dst)?;
    dst.truncate(offset);

    Ok((dst, version))
}

/// Encodes bytes into a prefixed Crockford Base32Check-encoded string.
///
/// # Errors
///
/// This method will return an [`Error`] if:
///
/// - [`Error::InvalidVersion`], the version is 32 or greater.
///
/// # Examples
///
/// ```rust
/// # use c32::Error;
/// let en = c32::encode_check_prefixed([42, 42, 42], 'S', 0)?;
/// assert_eq!(en, "S0AHA59B9201Z");
/// # Ok::<(), Error>(())
/// ```
#[inline]
#[cfg(all(feature = "alloc", feature = "check"))]
pub fn encode_check_prefixed<B>(src: B, prefix: char, version: u8) -> Result<String>
where
    B: AsRef<[u8]>,
{
    let src = src.as_ref();

    // Encode the input bytes.
    let encoded = encode_check(src, version)?;

    // Allocate the output string.
    let capacity = prefix.len_utf8() + encoded.len();
    let mut dst = String::with_capacity(capacity);

    // Append the prefix and encoded string.
    dst.push(prefix);
    dst.push_str(&encoded);
    Ok(dst)
}

/// Decodes a prefixed Crockford Base32Check-encoded string.
///
/// # Errors
///
/// This method will return an [`Error`] if:
///
/// - [`Error::MissingPrefix`], the input does not start with the prefix.
/// - [`Error::InvalidCharacter`], the input contains invalid characters.
/// - [`Error::InsufficientData`], the input has fewer bytes than required.
/// - [`Error::InvalidVersion`], the version is 32 or greater.
/// - [`Error::ChecksumMismatch`], the checksum's do not match.
///
/// # Examples
///
/// ```rust
/// # use c32::Error;
/// let (bytes, version) = c32::decode_check_prefixed("S0AHA59B9201Z", 'S')?;
/// assert_eq!(bytes, [42, 42, 42]);
/// assert_eq!(version, 0);
/// # Ok::<(), Error>(())
/// ```
#[inline]
#[cfg(all(feature = "alloc", feature = "check"))]
pub fn decode_check_prefixed(str: &str, prefix: char) -> Result<(Vec<u8>, u8)> {
    // Assert that the string starts with the prefix.
    if !str.starts_with(prefix) {
        return Err(Error::MissingPrefix {
            char: prefix,
            got: str.chars().next(),
        });
    }

    // Skip the prefix character and decode the rest.
    match decode_check(&str[prefix.len_utf8()..]) {
        Ok(bytes) => Ok(bytes),
        Err(Error::InvalidCharacter { char, index }) => {
            // This adjusts the index in an 'InvalidCharacter' to account for
            // the prefix in the original string that we don't decode.
            Err(Error::InvalidCharacter {
                char,
                index: index + prefix.len_utf8(),
            })
        }
        Err(e) => Err(e),
    }
}

/// Encodes bytes as Crockford Base32 into a provided buffer.
///
/// # Returns
///
/// The number of bytes written to the output buffer.
///
/// # Errors
///
/// This method will return an [`Error`] if:
///
/// - [`Error::BufferTooSmall`], the output buffer lacks capacity.
///
/// # Examples
///
/// ```rust
/// # use c32::Error;
/// # let mut dst = [0u8; 5];
/// let offset = c32::encode_into(&[42, 42, 42], &mut dst)?;
/// assert_eq!(&dst[..offset], b"2MAHA");
/// assert_eq!(offset, 5);
/// # Ok::<(), Error>(())
/// ```
#[inline]
pub fn encode_into(src: &[u8], dst: &mut [u8]) -> Result<usize> {
    // Assert that the buffer has enough capacity.
    let capacity = encoded_len(src.len());
    if dst.len() < capacity {
        return Err(Error::BufferTooSmall {
            min: capacity,
            len: dst.len(),
        });
    }

    // Encode the input bytes, and return the amount of bytes written.
    let offset = __internal::en(src, 0, src.len(), &mut dst[..], 0, None);
    Ok(offset)
}

/// Decodes Crockford Base32-encoded bytes into a provided buffer.
///
/// # Returns
///
/// The number of bytes written to the output buffer.
///
/// # Errors
///
/// This method will return an [`Error`] if:
///
/// - [`Error::BufferTooSmall`], the output buffer lacks capacity.
/// - [`Error::InvalidCharacter`], the input contains invalid characters.
///
/// # Examples
///
/// ```rust
/// # use c32::Error;
/// # let mut dst = [0u8; 5];
/// let offset = c32::decode_into(b"2MAHA", &mut dst)?;
/// assert_eq!(&dst[..offset], [42, 42, 42]);
/// assert_eq!(offset, 3);
/// # Ok::<(), Error>(())
/// ```
#[inline]
pub fn decode_into(src: &[u8], dst: &mut [u8]) -> Result<usize> {
    // Assert that the buffer has enough capacity.
    let capacity = decoded_len(src.len());
    if dst.len() < capacity {
        return Err(Error::BufferTooSmall {
            min: capacity,
            len: dst.len(),
        });
    }

    // Encode the input bytes, and return the amount of bytes written.
    __internal::de(src, 0, src.len(), dst, 0)
}

/// Encodes bytes as Crockford Base32Check into a provided buffer.
///
/// # Returns
///
/// The number of bytes written to the output buffer.
///
/// # Errors
///
/// This method will return an [`Error`] if:
///
/// - [`Error::BufferTooSmall`], the output buffer lacks capacity.
/// - [`Error::InvalidVersion`], the version is 32 or greater.
///
/// # Examples
///
/// ```rust
/// # use c32::Error;
/// # let mut dst = [0u8; 13];
/// let offset = c32::encode_check_into(&[42, 42, 42], &mut dst, 0)?;
/// assert_eq!(&dst[..offset], b"0AHA59B9201Z");
/// assert_eq!(offset, 12);
/// # Ok::<(), Error>(())
/// ```
#[inline]
#[cfg(feature = "check")]
pub fn encode_check_into(src: &[u8], dst: &mut [u8], ver: u8) -> Result<usize> {
    // Assert that the buffer has enough capacity.
    let capacity = encoded_check_len(src.len());
    if dst.len() < capacity {
        return Err(Error::BufferTooSmall {
            min: capacity,
            len: dst.len(),
        });
    }

    // Assert that the version is valid (>= 32).
    if ver >= 32 {
        return Err(Error::InvalidVersion {
            expected: "must be >= 32",
            version: ver,
        });
    }

    // Insert the version character into the output buffer.
    let mut offset = 0;
    dst[offset] = ALPHABET[ver as usize];
    offset += 1;

    // Compute the checksum for the input bytes and version.
    let sum = checksum::compute(src, ver);

    // Encode the bytes and checksum.
    offset += __internal::en(src, 0, src.len(), &mut dst[offset..], 0, Some(sum));

    Ok(offset)
}

/// Decodes Crockford Base32Check-encoded bytes into a provided buffer.
///
/// # Returns
///
/// The number of bytes written to the output buffer and the version.
///
/// # Errors
///
/// This method will return an [`Error`] if:
///
/// - [`Error::InvalidCharacter`], the input contains invalid characters.
/// - [`Error::InsufficientData`], the input has fewer bytes than required.
/// - [`Error::InvalidVersion`], the version is 32 or greater.
/// - [`Error::ChecksumMismatch`], the checksum's do not match.
/// - [`Error::BufferTooSmall`], the output buffer lacks capacity.
///
/// # Examples
///
/// ```rust
/// # use c32::Error;
/// # let mut dst = [0u8; 12];
/// let (offset, version) = c32::decode_check_into(b"0AHA59B9201Z", &mut dst)?;
/// assert_eq!(&dst[..offset], [42, 42, 42]);
/// assert_eq!(version, 0);
/// # Ok::<(), Error>(())
/// ```
#[inline]
#[cfg(feature = "check")]
#[allow(clippy::missing_panics_doc)]
pub fn decode_check_into(src: &[u8], dst: &mut [u8]) -> Result<(usize, u8)> {
    // Assert that the buffer has enough capacity.
    let capacity = decoded_check_len(src.len());
    if dst.len() < capacity {
        return Err(Error::BufferTooSmall {
            min: capacity,
            len: dst.len(),
        });
    }

    // Assert that the input bytes contain the minimum amount.
    if src.len() < 2 {
        return Err(Error::InsufficientData {
            min: 2,
            len: src.len(),
        });
    }

    // This should not panic, as the check above ensures enough bytes.
    let (tag, payload) = src.split_first().unwrap();

    // Decode the version byte.
    let mut buffer = [0u8; 1];
    let _ = __internal::de(&[*tag], 0, 1, &mut buffer, 0)?;
    let version = buffer[0];

    // Assert that the recovered version is valid. (>= 32).
    if version >= 32 {
        return Err(Error::InvalidVersion {
            expected: "must be >= 32",
            version,
        });
    }

    // Decode the remaining bytes into the output buffer.
    let mut offset = match __internal::de(payload, 0, payload.len(), dst, 0) {
        Ok(pos) => pos,
        Err(Error::InvalidCharacter { char, index }) => {
            return Err(Error::InvalidCharacter {
                char,
                index: index + 1,
            });
        }
        Err(e) => return Err(e),
    };

    // Extract the checksum.
    offset -= checksum::BYTE_LENGTH;
    let sum = checksum::from_slice(&dst[offset..offset + checksum::BYTE_LENGTH]);

    // Compute the expected checksum.
    let expected = checksum::compute(&dst[..offset], version);

    // Assert that the computed and actual checksums match.
    if !__internal::memcmp(&expected, &sum, checksum::BYTE_LENGTH) {
        return Err(Error::ChecksumMismatch { expected, got: sum });
    }

    Ok((offset, version))
}

/// Private module containing internal methods.
#[allow(dead_code)]
mod __internal {
    use super::*;

    /// Encodes a byte slice into Crockford Base32.
    ///
    /// # Notes
    ///
    /// The caller must ensure, that the output buffer is sufficiently sized.
    #[inline]
    #[must_use]
    pub(crate) const fn en(
        src: &[u8],
        src_offset: usize,
        src_len: usize,
        dst: &mut [u8],
        dst_offset: usize,
        checksum: Option<[u8; 4]>,
    ) -> usize {
        const MASK_5: u16 = 0x1F;
        const SHIFT_5: u16 = 5;

        let mut carry = 0;
        let mut carry_bits = 0;
        let mut dst_pos = dst_offset;

        // count leading zeros
        let mut leading_zeros = 0;
        while leading_zeros < src_len && src[src_offset + leading_zeros] == 0 {
            leading_zeros += 1;
        }

        // process checksum if provided
        if let Some(sum) = checksum {
            let mut checksum_pos = sum.len();
            while checksum_pos > 0 {
                checksum_pos -= 1;

                // accumulate bits into carry
                carry |= (sum[checksum_pos] as u16) << carry_bits;
                carry_bits += 8;

                // extract 5-bit chunks
                while carry_bits >= SHIFT_5 {
                    // write character from chunk
                    dst[dst_pos] = ALPHABET[(carry & MASK_5) as usize];
                    dst_pos += 1;

                    // shift out processed bytes
                    carry >>= SHIFT_5;
                    carry_bits -= SHIFT_5;
                }
            }
        }

        // process bytes in reverse
        let mut input_pos = src_offset + src_len;
        while input_pos > src_offset {
            input_pos -= 1;

            // accumulate bits into carry
            carry |= (src[input_pos] as u16) << carry_bits;
            carry_bits += 8;

            // extract 5-bit chunks
            while carry_bits >= SHIFT_5 {
                // write character from chunk
                dst[dst_pos] = ALPHABET[(carry & MASK_5) as usize];
                dst_pos += 1;

                // shift out processed bits
                carry >>= SHIFT_5;
                carry_bits -= SHIFT_5;
            }
        }

        // process remaining bits
        if carry_bits > 0 && carry > 0 {
            dst[dst_pos] = ALPHABET[(carry & MASK_5) as usize];
            dst_pos += 1;
        }

        // truncate trailing zeros
        while dst_pos > dst_offset && dst[dst_pos - 1] == ALPHABET[0] {
            dst_pos -= 1;
        }

        // restore leading zeros from input
        let mut j = 0;
        while j < leading_zeros {
            dst[dst_pos] = ALPHABET[0];
            dst_pos += 1;
            j += 1;
        }

        // reverse the buffer
        if dst_pos > dst_offset {
            let mut lhs = dst_offset;
            let mut rhs = dst_pos - 1;
            while lhs < rhs {
                let temp = dst[lhs];
                dst[lhs] = dst[rhs];
                dst[rhs] = temp;
                lhs += 1;
                rhs -= 1;
            }
        }

        dst_pos - dst_offset
    }

    /// Decodes Crockford Base32-encoded bytes.
    ///
    /// # Notes
    ///
    /// The caller must ensure, that the output buffer is sufficiently sized.
    #[inline]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub(crate) const fn de(
        src: &[u8],
        src_offset: usize,
        src_len: usize,
        dst: &mut [u8],
        dst_offset: usize,
    ) -> Result<usize> {
        const MASK_8: u16 = 0xFF;
        const SHIFT_8: u16 = 8;

        let mut carry = 0;
        let mut carry_bits = 0;
        let mut dst_pos = dst_offset;

        // count leading zeros
        let mut leading_zeros = 0;
        while leading_zeros < src_len && src[src_offset + leading_zeros] == ALPHABET[0] {
            leading_zeros += 1;
        }

        // process characters in reverse
        let mut input_pos = src_offset + src_len;
        while input_pos > src_offset {
            input_pos -= 1;

            // fetch the byte
            let byte = src[input_pos];
            if byte >= 128 {
                return Err(Error::InvalidCharacter {
                    char: byte as char,
                    index: input_pos - src_offset,
                });
            }

            // convert the byte to a map index
            let index = BYTE_MAP[byte as usize];
            if index < 0 {
                return Err(Error::InvalidCharacter {
                    char: byte as char,
                    index: input_pos - src_offset,
                });
            }

            // accumulate bits into carry
            carry |= (index as u16) << carry_bits;
            carry_bits += 5;

            // extract 8-bit chunks
            while carry_bits >= SHIFT_8 {
                // write byte from chunk
                dst[dst_pos] = (carry & MASK_8) as u8;
                dst_pos += 1;

                // shift out processed bits
                carry >>= SHIFT_8;
                carry_bits -= SHIFT_8;
            }
        }

        // process remaining bits
        if carry_bits > 0 {
            dst[dst_pos] = carry as u8;
            dst_pos += 1;
        }

        // truncate trailing zeros
        while dst_pos > dst_offset && dst[dst_pos - 1] == 0 {
            dst_pos -= 1;
        }

        // restore leading zeros from input
        let mut j = 0;
        while j < leading_zeros {
            dst[dst_pos] = 0;
            dst_pos += 1;
            j += 1;
        }

        // reverse the buffer
        if dst_pos > dst_offset {
            let mut lhs = dst_offset;
            let mut rhs = dst_pos - 1;
            while lhs < rhs {
                let temp = dst[lhs];
                dst[lhs] = dst[rhs];
                dst[rhs] = temp;
                lhs += 1;
                rhs -= 1;
            }
        }

        Ok(dst_pos - dst_offset)
    }

    /// Copies `n` bytes from `src` to `dst`.
    ///
    /// # Notes
    ///
    /// - Both slices must be properly sized.
    #[inline]
    pub(crate) const fn memcpy(
        dst: &mut [u8],
        dst_offset: usize,
        src: &[u8],
        src_offset: usize,
        n: usize,
    ) {
        let mut i = 0;
        while i < n {
            dst[dst_offset + i] = src[src_offset + i];
            i += 1;
        }
    }

    /// Compares `n` bytes between `lhs` and `rhs`.
    ///
    /// # Notes
    ///
    /// - Both slices must be properly sized.
    #[inline]
    #[must_use]
    pub(crate) const fn memcmp(lhs: &[u8], rhs: &[u8], n: usize) -> bool {
        let mut i = 0;
        while i < n {
            if lhs[i] != rhs[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}
