// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#[path = "macro.rs"]
mod macros;

test_cases! {
   test_c32_unit,
   fn(bytes: impl AsRef<[u8]>, exp: &str) {
       let bytes = bytes.as_ref();

       let mut ebuffer = [0; 32];
       let pos = c32::encode_into(bytes, &mut ebuffer).unwrap();
       assert_eq!(&ebuffer[..pos], exp.as_bytes());

       let mut dbuffer = [0; 32];
       let pos = c32::decode_into(exp.as_bytes(), &mut dbuffer).unwrap();
       assert_eq!(&dbuffer[..pos], bytes);
   },
   "empty": ([], ""),
   "byte_zero": ([0], "0"),
   "byte_single": ([1], "1"),
   "byte_quarter": ([64], "20"),
   "byte_mid": ([127], "3Z"),
   "byte_thirds": ([192], "60"),
   "byte_max": ([255], "7Z"),
   "pair_zero": ([0, 0], "00"),
   "pair_asc": ([1, 2], "82"),
   "pair_desc": ([2, 1], "G1"),
   "pair_max": ([255, 255], "1ZZZ"),
   "pair_same": ([42, 42], "AHA"),
   "triple_zero": ([0, 0, 0], "000"),
   "triple_asc": ([1, 2, 3], "20G3"),
   "triple_desc": ([3, 2, 1], "60G1"),
   "triple_max": ([255, 255, 255], "FZZZZ"),
   "triple_same": ([42, 42, 42], "2MAHA"),
   "zeros_two": ([0, 0], "00"),
   "zeros_three": ([0, 0, 0], "000"),
   "zeros_four": ([0, 0, 0, 0], "0000"),
   "zeros_five": ([0, 0, 0, 0, 0], "00000"),
   "zeros_cons": ([0, 0, 1], "001"),
   "zero_mid": ([1, 0, 1], "2001"),
   "zero_trail": ([1, 0], "80"),
   "zero_lead": ([0, 1], "01"),
   "zeros_space": ([1, 0, 1, 0], "G0080"),
   "repeat_low": ([1, 1, 1, 1], "G2081"),
   "repeat_high": ([254, 254, 254, 254], "3ZFXZQY")
}

#[cfg(feature = "alloc")]
test_cases! {
    test_c32_unit_alloc,
    fn(bytes: impl AsRef<[u8]>, exp: &str) {
        let bytes = bytes.as_ref();
        assert_eq!(c32::encode(bytes), exp);
        assert_eq!(c32::decode(exp).unwrap(), bytes);
    },
    "empty": ([], ""),
    "byte_zero": ([0], "0"),
    "byte_single": ([1], "1"),
    "byte_quarter": ([64], "20"),
    "byte_mid": ([127], "3Z"),
    "byte_thirds": ([192], "60"),
    "byte_max": ([255], "7Z"),
    "pair_zero": ([0, 0], "00"),
    "pair_asc": ([1, 2], "82"),
    "pair_desc": ([2, 1], "G1"),
    "pair_max": ([255, 255], "1ZZZ"),
    "pair_same": ([42, 42], "AHA"),
    "triple_zero": ([0, 0, 0], "000"),
    "triple_asc": ([1, 2, 3], "20G3"),
    "triple_desc": ([3, 2, 1], "60G1"),
    "triple_max": ([255, 255, 255], "FZZZZ"),
    "triple_same": ([42, 42, 42], "2MAHA"),
    "zeros_two": ([0, 0], "00"),
    "zeros_three": ([0, 0, 0], "000"),
    "zeros_four": ([0, 0, 0, 0], "0000"),
    "zeros_five": ([0, 0, 0, 0, 0], "00000"),
    "zeros_cons": ([0, 0, 1], "001"),
    "zero_mid": ([1, 0, 1], "2001"),
    "zero_trail": ([1, 0], "80"),
    "zero_lead": ([0, 1], "01"),
    "zeros_space": ([1, 0, 1, 0], "G0080"),
    "repeat_low": ([1, 1, 1, 1], "G2081"),
    "repeat_high": ([254, 254, 254, 254], "3ZFXZQY")
}
