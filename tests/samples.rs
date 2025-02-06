// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use test_case::test_case;

#[test_case("samples/c32_m_100x32b.in";  "encode multi 32b")]
#[test_case("samples/c32_m_100x64b.in";  "encode multi 64b")]
#[test_case("samples/c32_m_100x128b.in"; "encode multi 128b")]
#[test_case("samples/c32_m_100x256b.in"; "encode multi 256b")]
#[test_case("samples/c32_m_100x512b.in"; "encode multi 512b")]
#[test_case("samples/c32_m_100x1k.in";   "encode multi 1k")]
#[test_case("samples/c32_m_100x2k.in";   "encode multi 2k")]
#[test_case("samples/c32_m_100x4k.in";   "encode multi 4k")]
#[test_case("samples/c32_s_32k.in";      "encode single 32k")]
#[test_case("samples/c32_s_64k.in";      "encode single 64k")]
#[test_case("samples/c32_s_128k.in";     "encode single 128k")]
#[test_case("samples/c32_s_256k.in";     "encode single 256k")]
#[test_case("samples/c32_s_512k.in";     "encode single 512k")]
#[test_case("samples/c32_s_1m.in";       "encode single 1m")]
#[test_case("samples/c32_s_2m.in";       "encode single 2m")]
#[test_case("samples/c32_s_4m.in";       "encode single 4m")]
fn test_c32_samples(path: &str) {
    let input = std::fs::read(path).unwrap();
    let mut ebuffer = vec![0; c32::encoded_len(input.len())];
    let pos = c32::encode_into(&input, &mut ebuffer).unwrap();

    let mut dbuffer = vec![0; c32::decoded_len(pos)];
    let pos = c32::decode_into(&ebuffer[..pos], &mut dbuffer).unwrap();

    assert_eq!(&dbuffer[..pos], &input);
}
