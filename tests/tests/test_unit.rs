// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use core::str;

use c32::decode;
use c32::decode_check;
use c32::decode_check_into;
use c32::decode_check_prefixed;
use c32::decode_into;
use c32::decode_prefixed;
use c32::decoded_check_len;
use c32::decoded_len;
use c32::encode;
use c32::encode_check;
use c32::encode_check_into;
use c32::encode_check_prefixed;
use c32::encode_into;
use c32::encode_prefixed;
use c32::encoded_check_len;
use c32::encoded_len;
use c32::Result;

mod __internal {
    use super::*;

    /// A test helper for default features.
    pub fn test_default(bytes: &[u8], str: &str) -> Result<()> {
        let mut ebuf = vec![0u8; encoded_len(bytes.len())];
        let epos = encode_into(&bytes, &mut ebuf)?;

        let mut dbuf = vec![0u8; decoded_len(epos)];
        let dpos = decode_into(&ebuf[..epos], &mut dbuf)?;
        assert_eq!(&dbuf[..dpos], bytes);

        let en = str::from_utf8(&ebuf[..epos]).unwrap();
        assert_eq!(en, str);
        Ok(())
    }

    /// A test helper for `[feature = "alloc"]`.
    pub fn test_alloc(bytes: &[u8], expected: &str) -> Result<()> {
        let en = encode(&bytes);
        assert_eq!(en, expected);
        let de = decode(&en)?;
        assert_eq!(de, bytes);
        Ok(())
    }

    /// A test helper for `[feature = "check"]`.
    pub fn test_check(bytes: &[u8], expected: &str) -> Result<()> {
        let mut ebuf = vec![0u8; encoded_check_len(bytes.len())];
        let epos = encode_check_into(&bytes, &mut ebuf, 0)?;

        let mut dbuf = vec![0u8; decoded_check_len(epos)];
        let (dpos, dver) = decode_check_into(&ebuf[..epos], &mut dbuf)?;
        assert_eq!(&dbuf[..dpos], bytes);
        assert_eq!(dver, 0);

        let en = str::from_utf8(&ebuf[..epos]).unwrap();
        assert_eq!(en, expected);
        Ok(())
    }

    /// A test helper for `[feature = "check"]` + `[feature = "alloc"]`.
    pub fn test_check_alloc(bytes: &[u8], expected: &str) -> Result<()> {
        let en = encode_check(&bytes, 0)?;
        assert_eq!(en, expected);
        println!("{en}");
        let (de, de_version) = decode_check(&en)?;
        assert_eq!(de_version, 0);
        assert_eq!(de, bytes);
        Ok(())
    }

    /// A test helper for prefixed encoding/decoding.
    pub fn test_prefixed(input: &[u8], expected: &str) -> Result<()> {
        let en = encode_prefixed(&input, 'S');
        assert!(en.starts_with('S'));
        assert!(en.ends_with(expected));
        let de = decode_prefixed(&en, 'S')?;
        assert_eq!(de, input);
        Ok(())
    }

    /// A test helper for `[feature = "check"]` prefixed encoding/decoding.
    pub fn test_check_prefixed(input: &[u8], expected: &str) -> Result<()> {
        let en = encode_check_prefixed(&input, 'S', 0)?;
        assert!(en.starts_with('S'));
        assert!(en.ends_with(expected));
        let (de, de_version) = decode_check_prefixed(&en, 'S')?;
        assert_eq!(de_version, 0);
        assert_eq!(de, input);
        Ok(())
    }
}

#[test]
fn test_empty() {
    let input = [];
    let expected = "";
    __internal::test_default(&input, expected).unwrap();
}

#[test]
fn test_empty_alloc() {
    let input = [];
    let expected = "";
    __internal::test_alloc(&input, expected).unwrap();
}

#[test]
fn test_empty_check() {
    let input = [];
    let expected = "0A0DR2R";
    __internal::test_check(&input, expected).unwrap();
}

#[test]
fn test_empty_check_alloc() {
    let input = [];
    let expected = "0A0DR2R";
    __internal::test_check_alloc(&input, expected).unwrap();
}

#[test]
fn test_empty_prefixed() {
    let input = [];
    let expected = "";
    __internal::test_prefixed(&input, expected).unwrap();
}

#[test]
fn test_empty_check_prefixed() {
    let input = [];
    let expected = "0A0DR2R";
    __internal::test_check_prefixed(&input, expected).unwrap();
}

#[test]
fn test_ascending_one() {
    let input = [1];
    let expected = "1";
    __internal::test_default(&input, expected).unwrap();
}

#[test]
fn test_ascending_one_alloc() {
    let input = [1];
    let expected = "1";
    __internal::test_alloc(&input, expected).unwrap();
}

#[test]
fn test_ascending_one_check() {
    let input = [1];
    let expected = "04C407K6";
    __internal::test_check(&input, expected).unwrap();
}

#[test]
fn test_ascending_one_check_alloc() {
    let input = [1];
    let expected = "04C407K6";
    __internal::test_check_alloc(&input, expected).unwrap();
}

#[test]
fn test_ascending_one_check_prefixed() {
    let input = [1];
    let expected = "04C407K6";
    __internal::test_check_prefixed(&input, expected).unwrap();
}

#[test]
fn test_ascending_one_prefixed() {
    let input = [1];
    let expected = "1";
    __internal::test_prefixed(&input, expected).unwrap();
}

#[test]
fn test_ascending_two() {
    let input = [1, 2];
    let expected = "82";
    __internal::test_default(&input, expected).unwrap();
}

#[test]
fn test_ascending_two_alloc() {
    let input = [1, 2];
    let expected = "82";
    __internal::test_alloc(&input, expected).unwrap();
}

#[test]
fn test_ascending_two_check() {
    let input = [1, 2];
    let expected = "0108TZKWMK";
    __internal::test_check(&input, expected).unwrap();
}

#[test]
fn test_ascending_two_check_alloc() {
    let input = [1, 2];
    let expected = "0108TZKWMK";
    __internal::test_check_alloc(&input, expected).unwrap();
}

#[test]
fn test_ascending_two_check_prefixed() {
    let input = [1, 2];
    let expected = "0108TZKWMK";
    __internal::test_check_prefixed(&input, expected).unwrap();
}

#[test]
fn test_ascending_two_prefixed() {
    let input = [1, 2];
    let expected = "82";
    __internal::test_prefixed(&input, expected).unwrap();
}

#[test]
fn test_ascending_three() {
    let input = [1, 2, 3];
    let expected = "20G3";
    __internal::test_default(&input, expected).unwrap();
}

#[test]
fn test_ascending_three_alloc() {
    let input = [1, 2, 3];
    let expected = "20G3";
    __internal::test_alloc(&input, expected).unwrap();
}

#[test]
fn test_ascending_three_check() {
    let input = [1, 2, 3];
    let expected = "0820FVT6NE0";
    __internal::test_check(&input, expected).unwrap();
}

#[test]
fn test_ascending_three_check_alloc() {
    let input = [1, 2, 3];
    let expected = "0820FVT6NE0";
    __internal::test_check_alloc(&input, expected).unwrap();
}

#[test]
fn test_ascending_three_prefixed() {
    let input = [1, 2, 3];
    let expected = "20G3";
    __internal::test_prefixed(&input, expected).unwrap();
}

#[test]
fn test_ascending_three_check_prefixed() {
    let input = [1, 2, 3];
    let expected = "0820FVT6NE0";
    __internal::test_check_prefixed(&input, expected).unwrap();
}

#[test]
fn test_ascending_four() {
    let input = [1, 2, 3, 4];
    let expected = "G40R4";
    __internal::test_default(&input, expected).unwrap();
}

#[test]
fn test_ascending_four_alloc() {
    let input = [1, 2, 3, 4];
    let expected = "G40R4";
    __internal::test_alloc(&input, expected).unwrap();
}

#[test]
fn test_ascending_four_check() {
    let input = [1, 2, 3, 4];
    let expected = "020G30HV8M1Y1";
    __internal::test_check(&input, expected).unwrap();
}

#[test]
fn test_ascending_four_check_alloc() {
    let input = [1, 2, 3, 4];
    let expected = "020G30HV8M1Y1";
    __internal::test_check_alloc(&input, expected).unwrap();
}

#[test]
fn test_ascending_four_prefixed() {
    let input = [1, 2, 3, 4];
    let expected = "G40R4";
    __internal::test_prefixed(&input, expected).unwrap();
}

#[test]
fn test_ascending_four_check_prefixed() {
    let input = [1, 2, 3, 4];
    let expected = "020G30HV8M1Y1";
    __internal::test_check_prefixed(&input, expected).unwrap();
}

#[test]
fn test_ascending_five() {
    let input = [1, 2, 3, 4, 5];
    let expected = "4106105";
    __internal::test_default(&input, expected).unwrap();
}

#[test]
fn test_ascending_five_alloc() {
    let input = [1, 2, 3, 4, 5];
    let expected = "4106105";
    __internal::test_alloc(&input, expected).unwrap();
}

#[test]
fn test_ascending_five_check() {
    let input = [1, 2, 3, 4, 5];
    let expected = "0G40R40QP9HXK8";
    __internal::test_check(&input, expected).unwrap();
}

#[test]
fn test_ascending_five_check_alloc() {
    let input = [1, 2, 3, 4, 5];
    let expected = "0G40R40QP9HXK8";
    __internal::test_check_alloc(&input, expected).unwrap();
}

#[test]
fn test_ascending_five_prefixed() {
    let input = [1, 2, 3, 4, 5];
    let expected = "4106105";
    __internal::test_prefixed(&input, expected).unwrap();
}

#[test]
fn test_ascending_five_check_prefixed() {
    let input = [1, 2, 3, 4, 5];
    let expected = "0G40R40QP9HXK8";
    __internal::test_check_prefixed(&input, expected).unwrap();
}

#[test]
fn test_zero_all() {
    let input = [0, 0, 0, 0, 0];
    let expected = "00000";
    __internal::test_default(&input, expected).unwrap();
}

#[test]
fn test_zero_all_alloc() {
    let input = [0, 0, 0, 0, 0];
    let expected = "00000";
    __internal::test_alloc(&input, expected).unwrap();
}

#[test]
fn test_zero_all_check() {
    let input = [0, 0, 0, 0, 0];
    let expected = "0000001CKYF1A";
    __internal::test_check(&input, expected).unwrap();
}

#[test]
fn test_zero_all_check_alloc() {
    let input = [0, 0, 0, 0, 0];
    let expected = "0000001CKYF1A";
    __internal::test_check_alloc(&input, expected).unwrap();
}

#[test]
fn test_zero_all_prefixed() {
    let input = [0, 0, 0, 0, 0];
    let expected = "0";
    __internal::test_prefixed(&input, expected).unwrap();
}

#[test]
fn test_zero_all_check_prefixed() {
    let input = [0, 0, 0, 0, 0];
    let expected = "0000001CKYF1A";
    __internal::test_check_prefixed(&input, expected).unwrap();
}

#[test]
fn test_zero_alternating() {
    let input = [0, 1, 0, 1, 0];
    let expected = "0G0080";
    __internal::test_default(&input, expected).unwrap();
}

#[test]
fn test_zero_alternating_alloc() {
    let input = [0, 1, 0, 1, 0];
    let expected = "0G0080";
    __internal::test_alloc(&input, expected).unwrap();
}

#[test]
fn test_zero_alternating_check() {
    let input = [0, 1, 0, 1, 0];
    let expected = "00200100M9A7ZE";
    __internal::test_check(&input, expected).unwrap();
}

#[test]
fn test_zero_alternating_check_alloc() {
    let input = [0, 1, 0, 1, 0];
    let expected = "00200100M9A7ZE";
    __internal::test_check_alloc(&input, expected).unwrap();
}

#[test]
fn test_zero_alternating_prefixed() {
    let input = [0, 1, 0, 1, 0];
    let expected = "0G0080";
    __internal::test_prefixed(&input, expected).unwrap();
}

#[test]
fn test_zero_alternating_check_prefixed() {
    let input = [0, 1, 0, 1, 0];
    let expected = "00200100M9A7ZE";
    __internal::test_check_prefixed(&input, expected).unwrap();
}

#[test]
fn test_zero_middle() {
    let input = [0, 0, 1, 0, 0];
    let expected = "002000";
    __internal::test_default(&input, expected).unwrap();
}

#[test]
fn test_zero_middle_alloc() {
    let input = [0, 0, 1, 0, 0];
    let expected = "002000";
    __internal::test_alloc(&input, expected).unwrap();
}

#[test]
fn test_zero_middle_check() {
    let input = [0, 0, 1, 0, 0];
    let expected = "0008003VTB5H8";
    __internal::test_check(&input, expected).unwrap();
}

#[test]
fn test_zero_middle_check_alloc() {
    let input = [0, 0, 1, 0, 0];
    let expected = "0008003VTB5H8";
    __internal::test_check_alloc(&input, expected).unwrap();
}

#[test]
fn test_zero_middle_prefixed() {
    let input = [0, 0, 1, 0, 0];
    let expected = "002000";
    __internal::test_prefixed(&input, expected).unwrap();
}

#[test]
fn test_zero_middle_check_prefixed() {
    let input = [0, 0, 1, 0, 0];
    let expected = "0008003VTB5H8";
    __internal::test_check_prefixed(&input, expected).unwrap();
}

#[test]
fn test_zero_second() {
    let input = [0, 1, 0, 0, 0];
    let expected = "0G0000";
    __internal::test_default(&input, expected).unwrap();
}

#[test]
fn test_zero_second_alloc() {
    let input = [0, 1, 0, 0, 0];
    let expected = "0G0000";
    __internal::test_alloc(&input, expected).unwrap();
}

#[test]
fn test_zero_second_check() {
    let input = [0, 1, 0, 0, 0];
    let expected = "00200003Y4CA34";
    __internal::test_check(&input, expected).unwrap();
}

#[test]
fn test_zero_second_check_alloc() {
    let input = [0, 1, 0, 0, 0];
    let expected = "00200003Y4CA34";
    __internal::test_check_alloc(&input, expected).unwrap();
}

#[test]
fn test_zero_second_prefixed() {
    let input = [0, 1, 0, 0, 0];
    let expected = "0G0000";
    __internal::test_prefixed(&input, expected).unwrap();
}

#[test]
fn test_zero_second_check_prefixed() {
    let input = [0, 1, 0, 0, 0];
    let expected = "00200003Y4CA34";
    __internal::test_check_prefixed(&input, expected).unwrap();
}

#[test]
fn test_zero_two_ones_end() {
    let input = [0, 1, 0, 0, 1];
    let expected = "0G0001";
    __internal::test_default(&input, expected).unwrap();
}

#[test]
fn test_zero_two_ones_end_alloc() {
    let input = [0, 1, 0, 0, 1];
    let expected = "0G0001";
    __internal::test_alloc(&input, expected).unwrap();
}

#[test]
fn test_zero_two_ones_end_check() {
    let input = [0, 1, 0, 0, 1];
    let expected = "00200005F1PN5G";
    __internal::test_check(&input, expected).unwrap();
}

#[test]
fn test_zero_two_ones_end_check_alloc() {
    let input = [0, 1, 0, 0, 1];
    let expected = "00200005F1PN5G";
    __internal::test_check_alloc(&input, expected).unwrap();
}

#[test]
fn test_zero_two_ones_end_prefixed() {
    let input = [0, 1, 0, 0, 1];
    let expected = "0G0001";
    __internal::test_prefixed(&input, expected).unwrap();
}

#[test]
fn test_zero_two_ones_end_check_prefixed() {
    let input = [0, 1, 0, 0, 1];
    let expected = "00200005F1PN5G";
    __internal::test_check_prefixed(&input, expected).unwrap();
}
