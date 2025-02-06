// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use test_case::test_case;

#[test_case(&[],                   "";        "empty")]
#[test_case(&[0],                  "0";       "byte zero")]
#[test_case(&[64],                 "20";      "byte quarter")]
#[test_case(&[127],                "3Z";      "byte mid")]
#[test_case(&[192],                "60";      "byte thirds")]
#[test_case(&[255],                "7Z";      "byte max")]
#[test_case(&[0, 0],               "00";      "pair zero")]
#[test_case(&[1, 2],               "82";      "pair asc")]
#[test_case(&[2, 1],               "G1";      "pair desc")]
#[test_case(&[255, 255],           "1ZZZ";    "pair max")]
#[test_case(&[42, 42],             "AHA";     "pair same")]
#[test_case(&[0, 0, 0],            "000";     "triple zero")]
#[test_case(&[1, 2, 3],            "20G3";    "triple asc")]
#[test_case(&[3, 2, 1],            "60G1";    "triple desc")]
#[test_case(&[255, 255, 255],      "FZZZZ";   "triple max")]
#[test_case(&[42, 42, 42],         "2MAHA";   "triple same")]
#[test_case(&[0, 0],               "00";      "zeros two")]
#[test_case(&[0, 0, 0],            "000";     "zeros three")]
#[test_case(&[0, 0, 0, 0],         "0000";    "zeros four")]
#[test_case(&[0, 0, 0, 0, 0],      "00000";   "zeros five")]
#[test_case(&[0, 0, 1],            "001";     "zeros cons")]
#[test_case(&[1, 0, 1],            "2001";    "zero mid")]
#[test_case(&[1, 0],               "80";      "zero trail")]
#[test_case(&[0, 1],               "01";      "zero lead")]
#[test_case(&[1, 0, 1, 0],         "G0080";   "zeros space")]
#[test_case(&[1, 1, 1, 1],         "G2081";   "repeat low")]
#[test_case(&[254, 254, 254, 254], "3ZFXZQY"; "repeat high")]
fn test_c32_unit(bytes: &[u8], exp: &str) {
    let mut encode_buf = [0; 32];
    let written = c32::encode_into(bytes, &mut encode_buf).unwrap();
    assert_eq!(&encode_buf[..written], exp.as_bytes());

    let mut buffer = [0; 32];
    let pos = c32::decode_into(exp.as_bytes(), &mut buffer).unwrap();
    assert_eq!(&buffer[..pos], bytes);
}
