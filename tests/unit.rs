// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#[path = "macro.rs"]
mod macros;

gen_c32_unit_test!(test_c32_byte_min, vec![0], "0");
gen_c32_unit_test!(test_c32_byte_max, vec![255], "7Z");
gen_c32_unit_test!(test_c32_byte_mid, vec![127], "3Z");
gen_c32_unit_test!(test_c32_byte_quarter, vec![64], "20");
gen_c32_unit_test!(test_c32_byte_three_quarters, vec![192], "60");

gen_c32_unit_test!(test_c32_two_bytes_min, vec![0, 0], "00");
gen_c32_unit_test!(test_c32_two_bytes_max, vec![255, 255], "1ZZZ");
gen_c32_unit_test!(test_c32_two_bytes_ascending, vec![1, 2], "82");
gen_c32_unit_test!(test_c32_two_bytes_descending, vec![2, 1], "G1");
gen_c32_unit_test!(test_c32_two_bytes_same, vec![42, 42], "AHA");

gen_c32_unit_test!(test_c32_three_bytes_min, vec![0, 0, 0], "000");
gen_c32_unit_test!(test_c32_three_bytes_max, vec![255, 255, 255], "FZZZZ");
gen_c32_unit_test!(test_c32_three_bytes_ascending, vec![1, 2, 3], "20G3");
gen_c32_unit_test!(test_c32_three_bytes_descending, vec![3, 2, 1], "60G1");
gen_c32_unit_test!(test_c32_three_bytes_same, vec![42, 42, 42], "2MAHA");

gen_c32_unit_test!(test_c32_zero_length, vec![], "");
gen_c32_unit_test!(test_c32_zeros_one_byte, vec![0], "0");
gen_c32_unit_test!(test_c32_zeros_two_bytes, vec![0, 0], "00");
gen_c32_unit_test!(test_c32_zeros_three_bytes, vec![0, 0, 0], "000");
gen_c32_unit_test!(test_c32_zeros_four_bytes, vec![0, 0, 0, 0], "0000");
gen_c32_unit_test!(test_c32_zeros_five_bytes, vec![0, 0, 0, 0, 0], "00000");
gen_c32_unit_test!(test_c32_leading_zero, vec![0, 1], "01");
gen_c32_unit_test!(test_c32_trailing_zero, vec![1, 0], "80");
gen_c32_unit_test!(test_c32_middle_zero, vec![1, 0, 1], "2001");
gen_c32_unit_test!(test_c32_consecutive_zeros, vec![0, 0, 1], "001");
gen_c32_unit_test!(test_c32_spaced_zeros, vec![1, 0, 1, 0], "G0080");

gen_c32_unit_test!(test_c32_repeat_low, vec![1, 1, 1, 1], "G2081");
gen_c32_unit_test!(test_c32_repeat_high, vec![254, 254, 254, 254], "3ZFXZQY");
