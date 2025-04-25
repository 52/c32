// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use c32::decode;
use c32::decode_check;
use c32::decode_check_into;
use c32::decode_check_prefixed;
use c32::decode_into;
use c32::decode_prefixed;
use c32::encode_check;
use c32::encode_check_into;
use c32::encode_check_prefixed;
use c32::encode_into;
use c32::Error;

mod __internal {
    /// A test helper for [`Error::BufferTooSmall`] errors.
    macro_rules! assert_buffer_too_small {
        ($fn:expr, $min:expr, $len:expr) => {
            let result = $fn;
            assert!(matches!(
                result,
                Err(Error::BufferTooSmall { min, len }) if min == $min && len == $len
            ));
        };
    }

    /// A test helper for [`Error::InvalidCharacter`] errors.
    macro_rules! assert_invalid_character {
        ($fn:expr, $char:expr, $index:expr) => {
            let result = $fn;
            assert!(matches!(
                result,
                Err(Error::InvalidCharacter { char, index }) if char == $char && index == $index
            ));
        };
    }

    /// A test helper for [`Error::MissingPrefix`] errors.
    macro_rules! assert_missing_prefix {
        ($fn:expr, $char:expr, $got:expr) => {
            let result = $fn;
            assert!(matches!(
                result,
                Err(Error::MissingPrefix { char, got }) if char == $char && got == $got
            ));
        };
    }

    /// A test helper for [`Error::InvalidVersion`] errors.
    macro_rules! assert_invalid_version {
        ($fn:expr, $version:expr) => {
            let result = $fn;
            assert!(matches!(
                result,
                Err(Error::InvalidVersion { version }) if version == $version
            ));
        };
    }

    /// A test helper for [`Error::InsufficientData`] errors.
    macro_rules! assert_insufficient_data {
        ($fn:expr, $min:expr, $len:expr) => {
            let result = $fn;
            assert!(matches!(
                result,
                Err(Error::InsufficientData { min, len }) if min == $min && len == $len
            ));
        };
    }

    /// A test helper for [`Error::ChecksumMismatch`] errors.
    macro_rules! assert_checksum_mismatch {
        ($fn:expr) => {
            let result = $fn;
            assert!(matches!(
                result,
                Err(Error::ChecksumMismatch {
                    expected: _,
                    got: _
                })
            ));
        };
    }

    pub(crate) use assert_buffer_too_small;
    pub(crate) use assert_checksum_mismatch;
    pub(crate) use assert_insufficient_data;
    pub(crate) use assert_invalid_character;
    pub(crate) use assert_invalid_version;
    pub(crate) use assert_missing_prefix;
}

#[test]
fn test_error_encode_into_buffer_too_small() {
    let mut output = [0u8; 2];
    let result = encode_into(&[1, 2, 3, 4, 5], &mut output);
    __internal::assert_buffer_too_small!(result, 8, 2);
}

#[test]
fn test_error_decode_into_buffer_too_small() {
    let mut output = [0u8; 2];
    let result = decode_into(b"4106105", &mut output);
    __internal::assert_buffer_too_small!(result, 7, 2);
}

#[test]
fn test_error_encode_check_into_buffer_too_small() {
    let mut output = [0u8; 5];
    let result = encode_check_into(&[1, 2, 3], &mut output, 0);
    __internal::assert_buffer_too_small!(result, 13, 5);
}

#[test]
fn test_error_decode_check_into_buffer_too_small() {
    let mut output = [0u8; 2];
    let result = decode_check_into(b"0G40R40QP9HXK8", &mut output);
    __internal::assert_buffer_too_small!(result, 14, 2);
}

#[test]
fn test_error_decode_into_invalid_character() {
    let mut output = [0u8; 10];
    let result = decode_into(b"!MAHA", &mut output);
    __internal::assert_invalid_character!(result, '!', 0);
}

#[test]
fn test_error_decode_invalid_character() {
    let result = decode("!MAHA");
    __internal::assert_invalid_character!(result, '!', 0);
}

#[test]
fn test_error_decode_prefixed_invalid_character() {
    let result = decode_prefixed("S!MAHA", 'S');
    __internal::assert_invalid_character!(result, '!', 1);
}

#[test]
fn test_error_decode_check_into_invalid_character() {
    let mut output = [0u8; 32];
    let result = decode_check_into(b"0%AHA59B9201Z", &mut output);
    __internal::assert_invalid_character!(result, '%', 1);
}

#[test]
fn test_error_decode_check_invalid_character() {
    let result = decode_check("0!AHA59B9201Z");
    __internal::assert_invalid_character!(result, '!', 1);
}

#[test]
fn test_error_decode_check_prefixed_invalid_character() {
    let result = decode_check_prefixed("S0!AHA59B9201Z", 'S');
    __internal::assert_invalid_character!(result, '!', 2);
}

#[test]
fn test_error_decode_prefixed_missing_prefix() {
    let result = decode_prefixed("2MAHA", 'S');
    __internal::assert_missing_prefix!(result, 'S', Some('2'));
}

#[test]
fn test_error_decode_prefixed_empty_string() {
    let result = decode_prefixed("", 'S');
    __internal::assert_missing_prefix!(result, 'S', None);
}

#[test]
fn test_error_decode_check_prefixed_missing_prefix() {
    let result = decode_check_prefixed("0AHA59B9201Z", 'S');
    __internal::assert_missing_prefix!(result, 'S', Some('0'));
}

#[test]
fn test_error_decode_check_prefixed_empty_string() {
    let result = decode_check_prefixed("", 'S');
    __internal::assert_missing_prefix!(result, 'S', None);
}

#[test]
fn test_error_encode_check_into_invalid_version() {
    let mut output = [0u8; 32];
    let result = encode_check_into(&[1, 2, 3], &mut output, 32);
    __internal::assert_invalid_version!(result, 32);
}

#[test]
fn test_error_encode_check_invalid_version() {
    let result = encode_check(&[1, 2, 3], 32);
    __internal::assert_invalid_version!(result, 32);
}

#[test]
fn test_error_encode_check_prefixed_invalid_version() {
    let result = encode_check_prefixed(&[1, 2, 3], 'S', 32);
    __internal::assert_invalid_version!(result, 32);
}

#[test]
fn test_error_decode_check_into_insufficient_data() {
    let mut output = [0u8; 10];
    let result = decode_check_into(b"0", &mut output);
    __internal::assert_insufficient_data!(result, 2, 1);
}

#[test]
fn test_error_decode_check_insufficient_data() {
    let result = decode_check("0");
    __internal::assert_insufficient_data!(result, 2, 1);
}

#[test]
fn test_error_decode_check_prefixed_insufficient_data() {
    let result = decode_check_prefixed("S0", 'S');
    __internal::assert_insufficient_data!(result, 2, 1);
}

#[test]
fn test_error_decode_check_into_checksum_mismatch() {
    let mut output = [0u8; 10];
    let result = decode_check_into(b"04C407K7", &mut output);
    __internal::assert_checksum_mismatch!(result);
}

#[test]
fn test_error_decode_check_checksum_mismatch() {
    let result = decode_check("0820FVT6NE1");
    __internal::assert_checksum_mismatch!(result);
}

#[test]
fn test_error_decode_check_prefixed_checksum_mismatch() {
    let result = decode_check_prefixed("S0820FVT6NE1", 'S');
    __internal::assert_checksum_mismatch!(result);
}
