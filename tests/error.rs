// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#[path = "macro.rs"]
mod macros;

gen_c32_error_test!(
    test_c32_error_invalid_char_exclamation,
    "!",
    c32::C32Error::InvalidChar('!')
);

gen_c32_error_test!(
    test_c32_error_invalid_char_pound,
    "#",
    c32::C32Error::InvalidChar('#')
);

gen_c32_error_test!(
    test_c32_error_invalid_char_dollar,
    "$",
    c32::C32Error::InvalidChar('$')
);

gen_c32_error_test!(
    test_c32_error_invalid_char_percent,
    "%",
    c32::C32Error::InvalidChar('%')
);

gen_c32_error_test!(
    test_c32_error_invalid_char_caret,
    "^",
    c32::C32Error::InvalidChar('^')
);

gen_c32_error_test!(
    test_c32_error_invalid_char_ampersand,
    "&",
    c32::C32Error::InvalidChar('&')
);

gen_c32_error_test!(
    test_c32_error_invalid_char_asterisk,
    "*",
    c32::C32Error::InvalidChar('*')
);

gen_c32_error_test!(
    test_c32_error_non_ascii_japanese,
    "あ",
    c32::C32Error::InvalidString
);

gen_c32_error_test!(
    test_c32_error_non_ascii_emoji,
    "😀",
    c32::C32Error::InvalidString
);

gen_c32_error_test!(
    test_c32_error_non_ascii_chinese,
    "你",
    c32::C32Error::InvalidString
);

gen_c32_error_test!(
    test_c32_error_mixed_invalid,
    "ABC!DEF",
    c32::C32Error::InvalidChar('!')
);

gen_c32_error_test!(
    test_c32_error_mixed_non_ascii,
    "ABCあDEF",
    c32::C32Error::InvalidString
);
