// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#![deny(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]
#![allow(clippy::doc_markdown)]

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
//! # Examples
//!
//! #### `std` or `alloc`
//!
//! ```rust
//! # #[cfg(feature = "alloc")] {
//! // encoding...
//! let bytes = b"usque ad finem";
//! let encoded = c32::encode(&bytes);
//! assert_eq!(encoded, "1TQ6WBNCMG62S10CSMPWSBD");
//! # }
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! ```rust
//! # #[cfg(feature = "alloc")] {
//! // decoding...
//! let bytes = b"usque ad finem";
//! let decoded = c32::decode("1TQ6WBNCMG62S10CSMPWSBD")?;
//! assert_eq!(decoded, bytes);
//! # }
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! #### `#![no_std]`
//!
//! ```rust
//! // encoding...
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
//! // decoding...
//! let encoded = b"1TQ6WBNCMG62S10CSMPWSBD";
//! let mut buffer = [0; 32];
//!
//! let written = c32::decode_into(encoded, &mut buffer)?;
//! let decoded = &buffer[..written];
//! assert_eq!(decoded, b"usque ad finem");
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! # Checksums (`check`)
//!
//! The `check` feature provides methods for encoding data with SHA256-based
//! checksum verification.
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
//! ## Examples
//!
//! #### `std` or `alloc`
//!
//! ```rust
//! # #[cfg(all(feature = "check", feature = "alloc"))] {
//! // encoding...
//! let bytes = b"usque ad finem";
//! let encoded = c32::encode_check(bytes, 22)?;
//! assert_eq!(encoded, "P7AWVHENJJ0RB441K6JVK5DNJ7J3V5");
//! # }
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! ```rust
//! # #[cfg(all(feature = "check", feature = "alloc"))] {
//! // decoding...
//! let encoded = "P7AWVHENJJ0RB441K6JVK5DNJ7J3V5";
//! let (version, decoded) = c32::decode_check(encoded)?;
//! assert_eq!(decoded, b"usque ad finem");
//! assert_eq!(version, 22);
//! # }
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! #### `#![no_std]`
//!
//! ```rust
//! # #[cfg(feature = "check")] {
//! // encoding...
//! let bytes = b"usque ad finem";
//! let mut buffer = [0; 32];
//!
//! let written = c32::encode_check_into(bytes, 22, &mut buffer)?;
//! let encoded = &buffer[..written];
//! assert_eq!(encoded, b"P7AWVHENJJ0RB441K6JVK5DNJ7J3V5");
//! # }
//! # Ok::<(), c32::Error>(())
//! ```
//!
//! ```rust
//! # #[cfg(feature = "check")] {
//! // decoding...
//! let encoded = b"P7AWVHENJJ0RB441K6JVK5DNJ7J3V5";
//! let mut buffer = [0; 32];
//!
//! let (version, written) = c32::decode_check_into(encoded, &mut buffer)?;
//! let decoded = &buffer[..written];
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
//!  `std`   | Implement `std::error::Error` for [`Error`]
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

#[cfg(feature = "std")]
extern crate std;

/// Re-exports for `std` & `alloc` compatibility.
///
/// This module provides a unified interface for common allocation types.
pub(crate) mod lib {
    #[cfg(feature = "std")]
    mod core {
        pub use std::borrow::Cow;
        pub use std::string::String;
        pub use std::vec;
        pub use std::vec::Vec;
    }

    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    mod core {
        pub use alloc::borrow::Cow;
        pub use alloc::string::String;
        pub use alloc::vec;
        pub use alloc::vec::Vec;
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    pub use core::*;
}

/// Checksum computation and validation for Crockford Base32Check encoding.
///
/// This module provides functionality for generating 4-byte checksums.
#[cfg(feature = "check")]
pub mod checksum {
    use sha2::Digest;
    use sha2::Sha256;

    /// Length of the checksum in bytes.
    pub const BYTE_LENGTH: usize = 4;

    /// Type alias for a checksum.
    pub type Checksum = [u8; BYTE_LENGTH];

    /// Computes a 4-byte checksum from input bytes and a version number.
    ///
    /// The checksum is computed by:
    /// 1. Concatenating the version byte with the payload bytes.
    /// 2. Taking the SHA256 hash of the concatenated bytes.
    /// 3. Taking the SHA256 hash of the result.
    /// 4. Using the first 4 bytes as the checksum.
    ///
    /// # Arguments
    /// * `bytes` - The input bytes to compute the checksum for.
    /// * `version` - A version byte to prepend to the input bytes.
    ///
    /// # Returns
    /// A 4-byte array containing the computed checksum.
    pub fn compute<B>(bytes: B, version: u8) -> Checksum
    where
        B: AsRef<[u8]>,
    {
        let bytes = bytes.as_ref();
        let buffer = Sha256::new()
            .chain_update([version])
            .chain_update(bytes)
            .finalize();

        let mut checksum = [0u8; BYTE_LENGTH];
        checksum.copy_from_slice(&Sha256::digest(buffer)[..BYTE_LENGTH]);
        checksum
    }

    /// Creates a checksum from a slice of bytes.
    ///
    /// # Returns
    /// A 4-byte array containing the first 4 bytes from the input.
    ///
    /// # Panics
    /// Panics if the input slice contains fewer than 4 bytes.
    pub(crate) fn from_slice<B>(bytes: B) -> Checksum
    where
        B: AsRef<[u8]>,
    {
        let bytes = bytes.as_ref();
        let mut checksum = [0u8; BYTE_LENGTH];
        checksum.copy_from_slice(&bytes[..BYTE_LENGTH]);
        checksum
    }
}

/// Crockford Base32 alphabet, used for encoding/decoding.
pub(crate) const C32_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// Crockford Base32 byte map, used for lookup of values.
pub(crate) const C32_BYTE_MAP: [i8; 128] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1,
    -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20,
    21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, -1,
    10, 11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0, 22, 23, 24, 25,
    26, -1, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1,
];

/// Error variants for Crockford Base32 encoding and decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Attempted to decode an invalid Crockford-encoded string.
    InvalidString,
    /// Encountered a character that is not present in the `C32_ALPHABET`.
    InvalidChar(char),
    /// Computed checksum does not match expected checksum.
    #[cfg(feature = "check")]
    InvalidChecksum(checksum::Checksum, checksum::Checksum),
    /// Version must be less than or equal to 32.
    #[cfg(feature = "check")]
    InvalidVersion(u8),
    /// Data needs for an operation is not met.
    #[cfg(feature = "check")]
    InvalidDataSize(usize, usize),
    /// Buffer does not have enough capacity.
    InvalidBufferSize(usize, usize),
    /// Conversion from a integer failed.
    TryFromInt(core::num::TryFromIntError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidString => {
                write!(f, "String must contain only valid ASCII characters")
            }
            Self::InvalidChar(char) => {
                write!(f, "Character '{char}' is not a valid C32 character")
            }
            #[cfg(feature = "check")]
            Self::InvalidChecksum(comp, exp) => {
                write!(f, "Checksum validation failed: computed {comp:?}, expected {exp:?}")
            }
            #[cfg(feature = "check")]
            Self::InvalidVersion(version) => {
                write!(f, "Version must be <= 32, got {version}")
            }
            #[cfg(feature = "check")]
            Self::InvalidDataSize(recv, min) => {
                write!(f, "Not enough data: received '{recv}' bytes, minimum required is '{min}'")
            }
            Self::InvalidBufferSize(recv, min) => {
                write!(f, "Not enough buffer capacity: received '{recv}', minimum required is '{min}'")
            }
            Self::TryFromInt(err) => {
                write!(f, "{err}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<core::num::TryFromIntError> for Error {
    fn from(err: core::num::TryFromIntError) -> Self {
        Self::TryFromInt(err)
    }
}

/// Result type for fallible Crockford Base32 operations.
pub(crate) type Result<T> = core::result::Result<T, Error>;

/// Computes the required buffer capacity for encoding into Crockford Base32.
///
/// # Examples
///
/// ```rust
/// assert_eq!(c32::encoded_len(0), 0);
/// assert_eq!(c32::encoded_len(1), 2);
/// assert_eq!(c32::encoded_len(3), 5);
/// ```
#[must_use]
pub const fn encoded_len(len: usize) -> usize {
    (len * 8 + 4) / 5
}

/// Encodes bytes as Crockford Base32 into a provided output buffer.
///
/// # Returns
/// - `Ok(usize)`: The number of bytes written to the output buffer.
/// - `Err(Error)`: If any errors occur during the encoding process.
///
/// # Errors
/// - [`Error::InvalidBufferSize`] if the output buffer is too small.
///
/// # Examples
/// ```rust
/// let bytes = b"usque ad finem";
///
/// // allocate a buffer with enough capacity
/// let mut buffer = [0; 32];
///
/// // encode bytes into the buffer
/// let written = c32::encode_into(bytes, &mut buffer)?;
///
/// let expected = b"1TQ6WBNCMG62S10CSMPWSBD";
/// assert_eq!(&buffer[..written], expected);
/// # Ok::<(), c32::Error>(())
/// ```
pub fn encode_into<'a, B>(bytes: B, output: &mut [u8]) -> Result<usize>
where
    B: Clone + IntoIterator<Item = &'a u8>,
    B::IntoIter: DoubleEndedIterator,
{
    // for 5-bit chunks
    const MASK_5: u32 = 0x1F;
    const SHIFT_5: u32 = 5;

    let mut carry = 0;
    let mut carry_bits = 0;
    let mut output_pos = 0;

    // process bytes in reverse
    for byte in bytes.clone().into_iter().rev() {
        // accumulate bits into carry
        carry |= u32::from(*byte) << carry_bits;
        carry_bits += 8;

        // extract 5-bit chunks
        while carry_bits >= SHIFT_5 {
            // assert that we have enough capacity
            if output_pos >= output.len() {
                return Err(Error::InvalidBufferSize(
                    output.len(),
                    output_pos + 1,
                ));
            }

            // write character from chunk
            output[output_pos] = C32_ALPHABET[(carry & MASK_5) as usize];
            output_pos += 1;

            // shift out processed bits
            carry >>= SHIFT_5;
            carry_bits -= SHIFT_5;
        }
    }

    // process the remaining bits
    if carry_bits > 0 {
        output[output_pos] = C32_ALPHABET[(carry & MASK_5) as usize];
        output_pos += 1;
    }

    // truncate the trailing zeros
    while output_pos > 0 && output[output_pos - 1] == C32_ALPHABET[0] {
        output_pos -= 1;
    }

    // restore the leading zeros from the original input
    for _ in bytes.into_iter().take_while(|&&b| b == 0) {
        // assert that we have enough capacity
        if output_pos >= output.len() {
            return Err(Error::InvalidBufferSize(output.len(), output_pos + 1));
        }

        // write zero character to output
        output[output_pos] = C32_ALPHABET[0];
        output_pos += 1;
    }

    // reverse buffer to get correct byte order
    output[..output_pos].reverse();

    Ok(output_pos)
}

/// Computes the required capacity for encoding into Crockford Base32Check.
///
/// # Examples
///
/// ```rust
/// assert_eq!(c32::encoded_check_len(0), 8);
/// assert_eq!(c32::encoded_check_len(1), 9);
/// assert_eq!(c32::encoded_check_len(3), 13);
/// ```
#[must_use]
#[cfg(feature = "check")]
pub const fn encoded_check_len(len: usize) -> usize {
    1 + encoded_len(len + 4)
}

/// Encodes bytes as Crockford Base32Check into a provided output buffer.
///
/// # Returns
/// - `Ok(usize)`: The number of bytes written to the output buffer.
/// - `Err(Error)`: If any errors occur during the encoding process.
///
/// # Errors
/// * [`Error::InvalidVersion`] if the version >= 32.
/// - [`Error::InvalidBufferSize`] if the output buffer is too small.
///
/// # Examples
/// ```rust
/// let bytes = b"usque ad finem";
/// let version = 22;
///
/// // allocate a buffer with enough capacity
/// let mut buffer = [0; 32];
///
/// // encode bytes into the buffer
/// let written = c32::encode_check_into(bytes, version, &mut buffer)?;
///
/// let expected = b"P7AWVHENJJ0RB441K6JVK5DNJ7J3V5";
/// assert_eq!(&buffer[..written], expected);
/// # Ok::<(), c32::Error>(())
/// ```
#[cfg(feature = "check")]
pub fn encode_check_into<B>(
    bytes: B,
    version: u8,
    output: &mut [u8],
) -> Result<usize>
where
    B: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();

    // assert that the version is valid
    if version >= 32 {
        return Err(Error::InvalidVersion(version));
    }

    // assert that we have enough capacity
    let capacity = encoded_check_len(bytes.len());
    if output.len() < capacity {
        return Err(Error::InvalidBufferSize(output.len(), capacity));
    }

    // insert the version character
    let mut output_pos = 0;
    output[output_pos] = C32_ALPHABET[version as usize];
    output_pos += 1;

    // compute the input checksum
    let checksum = checksum::compute(bytes, version);

    // encode [bytes + checksum] into the buffer
    output_pos += encode_into(
        bytes.iter().chain(checksum.iter()),
        &mut output[output_pos..],
    )?;

    Ok(output_pos)
}

/// Encodes bytes into a Crockford Base32-encoded string.
///
/// # Panics
/// This function can panic in two cases:
/// - If encoding fails despite sufficient buffer capacity.
/// - If the encoded output contains non-UTF8 bytes.
///
/// All panics indicate implementation issues and should never occur.
///
/// # Examples
/// ```rust
/// let bytes = b"usque ad finem";
///
/// // encode bytes into a string
/// let encoded = c32::encode(bytes);
///
/// let expected = "1TQ6WBNCMG62S10CSMPWSBD";
/// assert_eq!(encoded, expected);
/// ```
#[cfg(feature = "alloc")]
pub fn encode<B>(bytes: B) -> lib::String
where
    B: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();

    // allocate the output buffer
    let capacity = encoded_len(bytes.len());
    let mut output = lib::vec![0; capacity];

    // SAFETY:
    // this should not panic as the buffer is allocated with enough capacity
    let written = encode_into(bytes, &mut output).unwrap();
    output.truncate(written);

    // SAFETY:
    // this should not panic as the output only contains ASCII characters
    lib::String::from_utf8(output).unwrap()
}

/// Encodes bytes into a Crockford Base32Check-encoded string.
///
/// # Errors
/// * [`Error::InvalidVersion`] if the version >= 32.
/// - [`Error::InvalidBufferSize`] if the output buffer is too small.
///
/// # Panics
/// This function can panic in one cases:
/// - If the encoded output contains non-UTF8 bytes.
///
/// All panics indicate implementation issues and should never occur.
///
/// # Examples
/// ```rust
/// let bytes = b"usque ad finem";
/// let version = 22;
///
/// // encode bytes with into a string
/// let encoded = c32::encode_check(bytes, version)?;
///
/// let expected = "P7AWVHENJJ0RB441K6JVK5DNJ7J3V5";
/// assert_eq!(encoded, expected);
/// # Ok::<(), c32::Error>(())
/// ```
#[cfg(all(feature = "check", feature = "alloc"))]
pub fn encode_check<B>(bytes: B, version: u8) -> Result<lib::String>
where
    B: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();

    // allocate the output buffer
    let capacity = encoded_check_len(bytes.len());
    let mut output = lib::vec![0; capacity];

    // encode into the output buffer
    let written = encode_check_into(bytes, version, &mut output)?;
    output.truncate(written);

    // SAFETY:
    // this should not panic as the output only contains ASCII characters
    Ok(lib::String::from_utf8(output).unwrap())
}

/// Computes the required capacity for decoding from Crockford Base32.
///
/// # Examples
///
/// ```rust
/// assert_eq!(c32::decoded_len(0), 0);
/// assert_eq!(c32::decoded_len(2), 2);
/// assert_eq!(c32::decoded_len(5), 5);
/// ```
#[must_use]
pub const fn decoded_len(len: usize) -> usize {
    len
}

/// Decodes Crockford Base32-encoded bytes into a provided output buffer.
///
/// # Returns
/// - `Ok(usize)`: The number of bytes written to the output buffer.
/// - `Err(Error)`: If any errors occur during the decoding process.
///
/// # Errors
/// - [`Error::InvalidString`] if the input contains non-ASCII characters.
/// - [`Error::InvalidChar`] if the character is not found in `C32_ALPHABET`.
/// - [`Error::InvalidBufferSize`] if the output buffer is too small.
/// - [`Error::TryFromInt`] when bit arithmetic operations exceeds bounds.
///
/// # Examples
/// ```rust
/// let bytes = b"1TQ6WBNCMG62S10CSMPWSBD";
///
/// // allocate a buffer with enough capacity
/// let mut buffer = [0; 32];
///
/// // decode bytes into the buffer
/// let written = c32::decode_into(bytes, &mut buffer)?;
///
/// let expected = b"usque ad finem";
/// assert_eq!(&buffer[..written], expected);
/// # Ok::<(), c32::Error>(())
/// ```
pub fn decode_into<B>(bytes: B, output: &mut [u8]) -> Result<usize>
where
    B: AsRef<[u8]>,
{
    // for 8-bit chunks
    const MASK_8: u32 = 0xFF;
    const SHIFT_8: u32 = 8;

    let bytes = bytes.as_ref();

    // return early if the bytes are empty
    if bytes.is_empty() {
        return Ok(0);
    }

    // assert that the bytes are ascii
    if !bytes.is_ascii() {
        return Err(Error::InvalidString);
    }

    // assert that we have enough capacity
    let capacity = decoded_len(bytes.len());
    if output.len() < capacity {
        return Err(Error::InvalidBufferSize(output.len(), capacity));
    }

    let mut carry = 0;
    let mut carry_bits = 0;
    let mut output_pos = 0;

    // process characters in reverse
    for char in bytes.iter().rev() {
        let index = C32_BYTE_MAP.get(*char as usize).copied().unwrap_or(-1);

        // assert that our character is present in `C32_BYTE_MAP`
        if index.is_negative() {
            return Err(Error::InvalidChar(*char as char));
        }

        // accumulate bits into carry
        carry |= u32::from(u8::try_from(index)?) << carry_bits;
        carry_bits += 5;

        // extract 8-bit chunks
        while carry_bits >= SHIFT_8 {
            // write the byte from chunk
            output[output_pos] = (carry & MASK_8) as u8;
            output_pos += 1;

            // shift out processed bits
            carry >>= SHIFT_8;
            carry_bits -= SHIFT_8;
        }
    }

    // process the remaining bits
    if carry_bits > 0 {
        output[output_pos] = u8::try_from(carry)?;
        output_pos += 1;
    }

    // truncate the trailing zeros
    while output_pos > 0 && output[output_pos - 1] == 0 {
        output_pos -= 1;
    }

    // restore the leading zeros from the original input
    let zeros = bytes.iter().take_while(|&&b| b == C32_ALPHABET[0]).count();
    if zeros > 0 {
        output[output_pos..output_pos + zeros].fill(0);
        output_pos += zeros;
    }

    // reverse buffer to get correct byte order
    output[..output_pos].reverse();

    Ok(output_pos)
}

/// Computes the required capacity for decoding from Crockford Base32Check.
///
/// # Examples
///
/// ```rust
/// assert_eq!(c32::decoded_check_len(0), 0);
/// assert_eq!(c32::decoded_check_len(2), 2);
/// assert_eq!(c32::decoded_check_len(5), 5);
/// ```
#[must_use]
#[cfg(feature = "check")]
pub const fn decoded_check_len(len: usize) -> usize {
    len
}

/// Decodes Crockford Base32Check bytes into a provided output buffer.
///
/// # Returns
/// - `Ok((u8, usize))`: The version byte and number of bytes written.
/// - `Err(Error)`: If any errors occur during the decoding process.
///
/// # Errors
/// - [`Error::InvalidString`] if the input contains non-ASCII characters.
/// - [`Error::InvalidChar`] if the character is not found in `C32_ALPHABET`.
/// - [`Error::InvalidBufferSize`] if the output buffer is too small.
/// - [`Error::InvalidDataSize`] when data requirements are not met.
/// - [`Error::TryFromInt`] when bit arithmetic operations exceeds bounds.
///
/// # Panics
/// This function can panic in two cases:
/// - If slice bounds are exceeded while computing the checksum.
/// - If the input bytes are empty when calling `split_first()`.
///
/// This panic indicates an implementation issue and should never occur.
///
/// # Examples
/// ```rust
/// let bytes = b"P7AWVHENJJ0RB441K6JVK5DNJ7J3V5";
///
/// // allocate a buffer with enough capacity
/// let mut buffer = [0; 32];
///
/// // decode bytes with checksum into the buffer
/// let (version, written) = c32::decode_check_into(bytes, &mut buffer)?;
///
/// let expected = b"usque ad finem";
/// assert_eq!(&buffer[..written], expected);
/// assert_eq!(version, 22);
/// # Ok::<(), c32::Error>(())
/// ```
#[cfg(feature = "check")]
pub fn decode_check_into<B>(bytes: B, output: &mut [u8]) -> Result<(u8, usize)>
where
    B: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();

    // assert the minimal byte length (version + bytes)
    if bytes.len() < 2 {
        return Err(Error::InvalidDataSize(bytes.len(), 2));
    }

    // SAFETY:
    // the length check above ensures we have enough bytes to split
    let (tag, bytes) = bytes.split_first().unwrap();

    // decode bytes + checksum into the output buffer
    let mut output_pos = decode_into(bytes, output)?;

    // assert that we wrote at least 4 bytes (checksum)
    if output_pos < checksum::BYTE_LENGTH {
        return Err(Error::InvalidDataSize(bytes.len(), 2));
    }

    // decode the version tag
    let mut buffer = [0; 1];
    decode_into([*tag], &mut buffer)?;
    let version = buffer[0];

    // compute the checksums (computed and expected)
    output_pos -= checksum::BYTE_LENGTH;
    let comp_checksum = checksum::compute(&output[..output_pos], version);
    let exp_checksum = checksum::from_slice(
        &output[output_pos..output_pos + checksum::BYTE_LENGTH],
    );

    // assert that both checksums match
    if comp_checksum != exp_checksum {
        return Err(Error::InvalidChecksum(comp_checksum, exp_checksum));
    }

    Ok((version, output_pos))
}

/// Decodes a Crockford Base32-encoded string.
///
/// # Returns
/// - `Ok(Vec<u8>)`: A vector containing the decoded bytes.
/// - `Err(Error)`: If any errors occur during the decoding process.
///
/// # Errors
/// - [`Error::InvalidString`] if the input contains non-ASCII characters.
/// - [`Error::InvalidChar`] if the character is not found in `C32_ALPHABET`.
/// - [`Error::InvalidBufferSize`] if the output buffer is too small.
/// - [`Error::TryFromInt`] when bit arithmetic operations exceeds bounds.
///
/// # Examples
/// ```rust
/// let encoded = "1TQ6WBNCMG62S10CSMPWSBD";
///
/// // decode string into a vector
/// let decoded = c32::decode(encoded)?;
///
/// let expected = b"usque ad finem";
/// assert_eq!(&decoded, expected);
/// # Ok::<(), c32::Error>(())
/// ```
#[cfg(feature = "alloc")]
pub fn decode<'a, S>(str: S) -> Result<lib::Vec<u8>>
where
    S: Into<lib::Cow<'a, str>>,
{
    let str = str.into();

    // allocate the output buffer
    let capacity = decoded_len(str.len());
    let mut output = lib::vec![0; capacity];

    // decode into the output buffer
    let written = decode_into(str.as_ref(), &mut output)?;
    output.truncate(written);

    Ok(output)
}

/// Decodes a Crockford Base32Check-encoded string.
///
/// # Returns
/// - `Ok((u8, Vec<u8>))`: The version byte and decoded bytes.
/// - `Err(Error)`: If any errors occur during the decoding process.
///
/// # Errors
/// - [`Error::InvalidString`] if the input contains non-ASCII characters.
/// - [`Error::InvalidChar`] if the character is not found in `C32_ALPHABET`.
/// - [`Error::InvalidBufferSize`] if the output buffer is too small.
/// - [`Error::InvalidDataSize`] when data requirements are not met.
/// - [`Error::TryFromInt`] when bit arithmetic operations exceeds bounds.
///
/// # Examples
/// ```rust
/// let encoded = "P7AWVHENJJ0RB441K6JVK5DNJ7J3V5";
///
/// // decode string with into a vector
/// let (version, decoded) = c32::decode_check(encoded)?;
///
/// let expected = b"usque ad finem";
/// assert_eq!(decoded, expected);
/// assert_eq!(version, 22);
/// # Ok::<(), c32::Error>(())
/// ```
#[cfg(all(feature = "check", feature = "alloc"))]
pub fn decode_check<'a, S>(str: S) -> Result<(u8, lib::Vec<u8>)>
where
    S: Into<lib::Cow<'a, str>>,
{
    let str = str.into();

    // allocate the output buffer
    let capacity = decoded_check_len(str.len());
    let mut output = lib::vec![0; capacity];

    // decode into the output buffer
    let (version, written) = decode_check_into(str.as_ref(), &mut output)?;
    output.truncate(written);

    Ok((version, output))
}
