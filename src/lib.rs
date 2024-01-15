// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

/// `C32` alphabet, used for encoding/decoding.
pub(crate) const C32_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// `C32` byte map, used for lookup of values.
pub(crate) const C32_BYTE_MAP: [u8; 128] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 255, 255, 255, 255, 255,
    255, 10, 11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0, 22, 23, 24,
    25, 26, 255, 27, 28, 29, 30, 31, 255, 255, 255, 255, 255, 255, 10, 11, 12,
    13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, 255, 27,
    28, 29, 30, 31, 255, 255, 255, 255, 255,
];

/// Error variants for `C32` encoding/decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum C32Error {
    /// Attempted to decode an invalid `C32` string.
    #[error("Invalid string, must be ASCII and contain only C32 characters")]
    InvalidString,
    /// Encountered a character that is not present in the `C32_ALPHABET`.
    #[error("Invalid character, '{0}' is not allowed in C32 encoding")]
    InvalidChar(char),
    /// Conversion from a integer failed.
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
}

/// Encodes raw bytes into their crockford base32 representation.
///
/// # Arguments
/// * `bytes` - `T: AsRef<[u8]>` representing the bytes to encode.
///
/// # Returns
/// An ASCII `String` containing the c32-encoded data.
///
/// # Panics
/// This function should never panic.
/// All characters are guaranteed to be valid ASCII from the `C32_ALPHABET` set.
///
/// # Example
///
/// ```rust
/// use c32::encode;
///
/// let bytes = b"semper prorsum";
/// let encoded = encode(bytes);
/// assert_eq!(encoded, "1SPAVBGCNS20W3JDXS76XBD");
/// ```
pub fn encode<T>(bytes: T) -> String
where
    T: AsRef<[u8]>,
{
    // for 5-bit chunks
    const MASK_5: u32 = 0x1F;
    const SHIFT_5: u32 = 5;

    let bytes = bytes.as_ref();

    // handle empty bytes
    if bytes.is_empty() {
        return String::new();
    }

    // pre-allocate buffer
    let pre_capacity = (bytes.len() * 8 + 4) / 5;
    let mut output = Vec::with_capacity(pre_capacity);

    let mut carry = 0;
    let mut carry_bits = 0;

    // process bytes in reverse
    for byte in bytes.iter().rev() {
        // accumulate bits into carry
        carry |= u32::from(*byte) << carry_bits;
        carry_bits += 8;

        // extract the first 5-bit chunk
        output.push(C32_ALPHABET[(carry & MASK_5) as usize]);
        carry >>= SHIFT_5;
        carry_bits -= SHIFT_5;

        // extract second 5-bit chunk (if possible)
        if carry_bits >= SHIFT_5 {
            output.push(C32_ALPHABET[(carry & MASK_5) as usize]);
            carry >>= SHIFT_5;
            carry_bits -= SHIFT_5;
        }
    }

    // process remaining bits
    if carry_bits > 0 {
        output.push(C32_ALPHABET[(carry & MASK_5) as usize]);
    }

    // remove trailing zeros
    while !output.is_empty() && output.last() == Some(&C32_ALPHABET[0]) {
        output.pop();
    }

    // restore leading zeros from original input
    let leading_zeros = bytes.iter().take_while(|&&b| b == 0).count();
    if leading_zeros > 0 {
        output.extend(std::iter::repeat(C32_ALPHABET[0]).take(leading_zeros));
    }

    // reverse buffer to get correct byte order
    output.reverse();

    // SAFETY:
    // The unwrap() cannot fail since we exclusively push characters
    // from `C32_ALPHABET`, which contains only valid ASCII characters
    String::from_utf8(output).unwrap()
}

/// Decodes a crockford base32 encoded string into bytes.
///
/// # Arguments
/// * `str` - `T: Into<Cow<'a, str>>` representing a c32-encoded string.
///
/// # Errors
///
/// Returns [`C32Error::InvalidString`] if the string contains:
/// - Non-ASCII characters.
///
/// Returns [`C32Error::InvalidChar`] if the string contains:
/// - Non-ASCII characters
/// - Characters not included in the [`C32_ALPHABET`](C32_ALPHABET)
///
/// Returns [`C32Error::TryFromInt`] if:
/// - Remaining bits overflow a byte during decoding.
///
/// # Example
///
/// ```rust
/// use c32::decode;
///
/// let str = "1SPAVBGCNS20W3JDXS76XBD";
/// let decoded = decode(str).unwrap();
/// assert_eq!(decoded, b"semper prorsum");
/// ```
pub fn decode<'a, T>(str: T) -> Result<Vec<u8>, C32Error>
where
    T: Into<std::borrow::Cow<'a, str>>,
{
    // for 8-bit chunks
    const MASK_8: u32 = 0xFF;
    const SHIFT_8: u32 = 8;

    let str = str.into();

    // handle empty string
    if str.is_empty() {
        return Ok(Vec::new());
    }

    // only allow ascii strings
    if !str.is_ascii() {
        return Err(C32Error::InvalidString);
    }

    // pre-allocate buffer
    let pre_capacity = (str.len() * 5 + 7) / 8;
    let mut output = Vec::with_capacity(pre_capacity);

    let mut carry = 0;
    let mut carry_bits = 0;

    // process characters in reverse
    for char in str.chars().rev() {
        let byte = C32_BYTE_MAP.get(char as usize).copied().unwrap_or(255);

        if byte == 255 {
            return Err(C32Error::InvalidChar(char));
        }

        carry |= u32::from(byte) << carry_bits;
        carry_bits += 5;

        // extract full bytes when possible
        while carry_bits >= SHIFT_8 {
            output.push((carry & MASK_8) as u8);
            carry >>= SHIFT_8;
            carry_bits -= SHIFT_8;
        }
    }

    // process remaining bits
    if carry_bits > 0 {
        output.push(u8::try_from(carry)?);
    }

    // remove leading zeros from output
    while !output.is_empty() && output.last() == Some(&0) {
        output.pop();
    }

    // add leading zeros from input
    let leading_zeros = str.chars().take_while(|&c| c == '0').count();
    if leading_zeros > 0 {
        output.extend(std::iter::repeat(0).take(leading_zeros));
    }

    // reverse buffer to get correct byte order
    output.reverse();

    Ok(output)
}
