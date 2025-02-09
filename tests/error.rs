// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use c32::Error;

#[path = "macro.rs"]
mod macros;

test_cases! {
    test_c32_encode_err,
    fn(bytes: impl AsRef<[u8]>, err: Error) {
        let mut buffer = [0; 1];
        let result = c32::encode_into(bytes.as_ref(), &mut buffer);
        assert_eq!(result.unwrap_err(), err);
    },
    "invalid_buffer_capacity": ([1, 1], Error::InvalidBufferSize(1, 2)),
}

test_cases! {
    test_c32_decode_err,
    fn(str: &str, err: Error) {
        let mut buffer = [0; 1];
        let result = c32::decode_into(str, &mut buffer);
        assert_eq!(result.unwrap_err(), err);
    },
    "invalid_buffer_capacity": ("AB", Error::InvalidBufferSize(1, 2)),
    "invalid_string": ("🫡", Error::InvalidString),
    "invalid_char": ("$", Error::InvalidChar('$')),
}
