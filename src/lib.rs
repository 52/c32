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

//! [![Crates.io](https://img.shields.io/crates/v/c32.svg)][Crates.io]
//! [![Documentation](https://docs.rs/c32/badge.svg)][Docs.rs]
//! [![Build Status](https://img.shields.io/github/actions/workflow/status/52/c32/rust.yml?branch=master)][Workflow]
//! [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)][License-Apache]
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)][License-MIT]
//!
//! Rust implementation of [Crockford's Base32][Crockford] encoding scheme.
//!
//! # Usage
//!
//! This crate provides two approaches for encoding/decoding:
//!
//! ### Allocation API
//!
//! ```rust
//! # #[cfg(feature = "alloc")] {
//! let bytes = b"usque ad finem";
//!
//! // encoding
//! let encoded = c32::encode(bytes);
//! assert_eq!(encoded, "1TQ6WBNCMG62S10CSMPWSBD");
//!
//! // decoding
//! let decoded = c32::decode(encoded).unwrap();
//! assert_eq!(decoded, bytes);
//! # }
//! ```
//!
//! ### Non-Allocation API (`no_std`)
//! ```rust
//! let bytes = b"usque ad finem";
//!
//! // encoding
//! let mut buffer = [0; 32];
//! let pos = c32::encode_into(bytes, &mut buffer).unwrap();
//! assert_eq!(&buffer[..pos], b"1TQ6WBNCMG62S10CSMPWSBD")
//! ```
//!
//! ```rust
//! let bytes = b"1TQ6WBNCMG62S10CSMPWSBD";
//!
//! // decoding
//! let mut buffer = [0; 32];
//! let pos = c32::decode_into(bytes, &mut buffer).unwrap();
//! assert_eq!(&buffer[..pos], b"usque ad finem")
//! ```
//!
//! # Features
//!
//! - `std` (default) - Enables standard library support.
//! - `alloc` - Allows usage in `no_std` environments with heap allocation.
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

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

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

/// Error variants for `C32` encoding/decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Attempted to decode an invalid `C32` string.
    InvalidString,
    /// Encountered a character that is not present in the `C32_ALPHABET`.
    InvalidChar(char),
    /// Buffer does not have enough capacity.
    BufferTooSmall(usize, usize),
    /// Conversion from a integer failed.
    TryFromInt(core::num::TryFromIntError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidString => {
                write!(f, "Invalid string, must be ASCII and contain only C32 characters")
            }
            Self::InvalidChar(char) => {
                write!(f, "Invalid character, '{char}' not recognized in C32")
            }
            Self::BufferTooSmall(rec, exp) => {
                write!(f, "Insufficient buffer capacity: {rec}, min: {exp}")
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

/// Result type for fallible C32 operations.
pub(crate) type Result<T> = core::result::Result<T, Error>;

/// Computes the required buffer capacity for encoding into Crockford's Base32.
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
/// - [`Error::BufferTooSmall`] if output buffer cannot hold the decoded data.
///
/// # Example
/// ```rust
/// let mut buffer = [0; 32];
/// let pos = c32::encode_into(b"usque ad finem", &mut buffer).unwrap();
/// assert_eq!(&buffer[..pos], b"1TQ6WBNCMG62S10CSMPWSBD")
/// ```
pub fn encode_into<B>(bytes: B, output: &mut [u8]) -> Result<usize>
where
    B: AsRef<[u8]>,
{
    // for 5-bit chunks
    const MASK_5: u32 = 0x1F;
    const SHIFT_5: u32 = 5;

    let bytes = bytes.as_ref();

    // handle empty bytes
    if bytes.is_empty() {
        return Ok(0);
    }

    // assert buffer capacity
    let capacity = encoded_len(bytes.len());
    if output.len() < capacity {
        return Err(Error::BufferTooSmall(output.len(), capacity));
    }

    let mut carry = 0;
    let mut carry_bits = 0;
    let mut output_pos = 0;

    // process bytes in reverse
    for byte in bytes.iter().rev() {
        // accumulate bits into carry
        carry |= u32::from(*byte) << carry_bits;
        carry_bits += 8;

        // extract 5-bit chunks
        while carry_bits >= SHIFT_5 {
            output[output_pos] = C32_ALPHABET[(carry & MASK_5) as usize];
            output_pos += 1;
            carry >>= SHIFT_5;
            carry_bits -= SHIFT_5;
        }
    }

    // process remaining bits
    if carry_bits > 0 {
        output[output_pos] = C32_ALPHABET[(carry & MASK_5) as usize];
        output_pos += 1;
    }

    // remove trailing zeros
    while output_pos > 0 && output[output_pos - 1] == C32_ALPHABET[0] {
        output_pos -= 1;
    }

    // restore leading zeros from original input
    let zeros = bytes.iter().take_while(|&&b| b == 0).count();
    output[output_pos..output_pos + zeros].fill(C32_ALPHABET[0]);
    output_pos += zeros;

    // reverse buffer to get correct byte order
    output[..output_pos].reverse();

    Ok(output_pos)
}

/// Encodes a byte sequence into a Crockford Base32-encoded `String`.
///
/// # Returns
/// A `String` containing the encoded C32 representation.
///
/// # Panics
/// Should not panic as we allocate the buffer with the required size.
///
/// # Example
/// ```rust
/// let encoded = c32::encode(b"usque ad finem");
/// assert_eq!(encoded, "1TQ6WBNCMG62S10CSMPWSBD")
/// ```
#[cfg(feature = "alloc")]
pub fn encode<B>(bytes: B) -> alloc::string::String
where
    B: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();

    // allocate buffer
    let capacity = encoded_len(bytes.len());
    let mut output = alloc::vec![0; capacity];

    // SAFETY:
    // should not panic as buffer is allocated with exact capacity
    let written = encode_into(bytes, &mut output).unwrap();
    output.truncate(written);

    // SAFETY:
    // should not panic as output only contains valid ASCII characters
    alloc::string::String::from_utf8(output).unwrap()
}

/// Computes the required buffer capacity for decoding from Crockford's Base32.
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

/// Decodes a Crockford Base32-encoded `String` into a provided output buffer.
///
/// # Returns
/// - `Ok(usize)`: The number of bytes written to the output buffer.
/// - `Err(Error)`: If any errors occur during the decoding process.
///
/// # Errors
/// - [`Error::InvalidString`] if input contains non-ASCII characters.
/// - [`Error::InvalidChar`] if character is not part of the C32 alphabet.
/// - [`Error::BufferTooSmall`] if output buffer cannot hold the decoded data.
/// - [`Error::TryFromInt`] when bit arithmetic operations exceeds bounds.
///
/// # Example
/// ```rust
/// let bytes = b"1TQ6WBNCMG62S10CSMPWSBD";
/// let mut buffer = [0; 32];
/// let pos = c32::decode_into(bytes, &mut buffer).unwrap();
/// assert_eq!(&buffer[..pos], b"usque ad finem")
/// ```
pub fn decode_into<B>(bytes: B, output: &mut [u8]) -> Result<usize>
where
    B: AsRef<[u8]>,
{
    // for 8-bit chunks
    const MASK_8: u32 = 0xFF;
    const SHIFT_8: u32 = 8;

    let bytes = bytes.as_ref();

    // handle empty string
    if bytes.is_empty() {
        return Ok(0);
    }

    // only allow ascii strings
    if !bytes.is_ascii() {
        return Err(Error::InvalidString);
    }

    // assert buffer capacity
    let capacity = decoded_len(bytes.len());
    if output.len() < capacity {
        return Err(Error::BufferTooSmall(output.len(), capacity));
    }

    let mut carry = 0;
    let mut carry_bits = 0;
    let mut output_pos = 0;

    // process characters in reverse
    for char in bytes.iter().rev() {
        let index = C32_BYTE_MAP.get(*char as usize).copied().unwrap_or(-1);

        if index.is_negative() {
            return Err(Error::InvalidChar(*char as char));
        }

        carry |= u32::from(u8::try_from(index)?) << carry_bits;
        carry_bits += 5;

        // extract 8-bit chunks
        while carry_bits >= SHIFT_8 {
            output[output_pos] = (carry & MASK_8) as u8;
            output_pos += 1;
            carry >>= SHIFT_8;
            carry_bits -= SHIFT_8;
        }
    }

    // process remaining bits
    if carry_bits > 0 {
        output[output_pos] = u8::try_from(carry)?;
        output_pos += 1;
    }

    // remove trailing zeros
    while output_pos > 0 && output[output_pos - 1] == 0 {
        output_pos -= 1;
    }

    // restore leading zeros from original input
    let zeros = bytes.iter().take_while(|&&b| b == C32_ALPHABET[0]).count();
    if zeros > 0 {
        output[output_pos..output_pos + zeros].fill(0);
        output_pos += zeros;
    }

    // reverse buffer to get correct byte order
    output[..output_pos].reverse();

    Ok(output_pos)
}

/// Decodes a Crockford Base32-encoded string into a `Vec<u8>`.
///
/// # Returns
/// - `Ok(Vec<u8>)`: A vector containing the decoded bytes.
/// - `Err(Error)`: If any errors occur during the decoding process.
///
/// # Errors
/// - [`Error::InvalidString`] if input contains non-ASCII characters.
/// - [`Error::InvalidChar`] if character is not part of the C32 alphabet.
/// - [`Error::BufferTooSmall`] if output buffer cannot hold the decoded data.
/// - [`Error::TryFromInt`] when bit arithmetic operations exceeds bounds.
///
/// # Example
/// ```rust
/// let decoded = c32::decode("1TQ6WBNCMG62S10CSMPWSBD").unwrap();
/// assert_eq!(&decoded, b"usque ad finem")
/// ```
#[cfg(feature = "alloc")]
pub fn decode<'a, S>(str: S) -> Result<alloc::vec::Vec<u8>>
where
    S: Into<alloc::borrow::Cow<'a, str>>,
{
    let str = str.into();

    // allocate buffer
    let capacity = decoded_len(str.len());
    let mut output = alloc::vec![0; capacity];

    // decode into output buffer
    let written = decode_into(str.as_ref(), &mut output)?;
    output.truncate(written);

    Ok(output)
}
