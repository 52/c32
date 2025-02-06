// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use c32::Error;
use test_case::test_case;

#[test_case("^",  Error::InvalidChar('^');     "invalid char caret")]
#[test_case("#",  Error::InvalidChar('#');     "invalid char pound")]
#[test_case("$",  Error::InvalidChar('$');     "invalid char dollar")]
#[test_case("%",  Error::InvalidChar('%');     "invalid char percent")]
#[test_case("*",  Error::InvalidChar('*');     "invalid char asterisk")]
#[test_case("&",  Error::InvalidChar('&');     "invalid char ampersand")]
#[test_case("!",  Error::InvalidChar('!');     "invalid char exclamation")]
#[test_case("AB", Error::BufferTooSmall(1, 2); "invalid buffer capacity")]
fn test_c32_decode_error(char: &str, err: Error) {
    assert_eq!(c32::decode_into(char, &mut [0; 1]).unwrap_err(), err);
}
