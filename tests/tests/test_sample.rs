// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::fs;

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
use rand::Rng;

mod __internal {
    use super::*;

    /// A test helper for default features.
    pub fn test_default(path: &str) -> Result<()> {
        let input = fs::read(path).unwrap();

        let mut ebuf = vec![0u8; encoded_len(input.len())];
        let epos = encode_into(&input, &mut ebuf)?;

        let mut dbuf = vec![0u8; decoded_len(epos)];
        let dpos = decode_into(&ebuf[..epos], &mut dbuf)?;

        assert_eq!(&dbuf[..dpos], input.as_slice());
        Ok(())
    }

    /// A test helper for `[feature = "alloc"]`.
    pub fn test_alloc(path: &str) -> Result<()> {
        let input = fs::read(path).unwrap();

        let en = encode(&input);
        let de = decode(&en)?;
        assert_eq!(de, input.as_slice());
        Ok(())
    }

    /// A test helper for `[feature = "check"]`.
    pub fn test_check(path: &str) -> Result<()> {
        let input = fs::read(path).unwrap();

        let mut rng = rand::rng();
        let version = rng.random_range(0..32);

        let mut ebuf = vec![0u8; encoded_check_len(input.len())];
        let epos = encode_check_into(&input, &mut ebuf, version)?;

        let mut dbuf = vec![0u8; decoded_check_len(epos)];
        let (dpos, dver) = decode_check_into(&ebuf[..epos], &mut dbuf)?;

        assert_eq!(&dbuf[..dpos], input.as_slice());
        assert_eq!(dver, version);
        Ok(())
    }

    /// A test helper for `[feature = "check"]` + `[feature = "alloc"]`.
    pub fn test_check_alloc(path: &str) -> Result<()> {
        let input = fs::read(path).unwrap();

        let mut rng = rand::rng();
        let version = rng.random_range(0..32);

        let en = encode_check(&input, version)?;
        let (de, de_version) = decode_check(&en)?;

        assert_eq!(de, input.as_slice());
        assert_eq!(de_version, version);
        Ok(())
    }

    /// A test helper for prefixed encoding/decoding.
    pub fn test_prefixed(path: &str) -> Result<()> {
        let input = fs::read(path).unwrap();

        let mut rng = rand::rng();
        let prefix = rng.random::<char>();

        let en = encode_prefixed(&input, prefix);
        let de = decode_prefixed(&en, prefix)?;

        assert_eq!(de, input.as_slice());
        Ok(())
    }

    /// A test helper for `[feature = "check"]` prefixed encoding/decoding.
    pub fn test_check_prefixed(path: &str) -> Result<()> {
        let input = fs::read(path).unwrap();

        let mut rng = rand::rng();
        let version = rng.random_range(0..32);
        let prefix = rng.random::<char>();

        let en = encode_check_prefixed(&input, prefix, version)?;
        let (de, de_version) = decode_check_prefixed(&en, prefix)?;

        assert_eq!(de, input.as_slice());
        assert_eq!(de_version, version);
        Ok(())
    }
}

#[test]
fn test_sample_multi_32b() {
    const PATH: &str = "../samples/c32_m_100x32b.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_multi_32b_alloc() {
    const PATH: &str = "../samples/c32_m_100x32b.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_32b_check() {
    const PATH: &str = "../samples/c32_m_100x32b.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_multi_32b_check_alloc() {
    const PATH: &str = "../samples/c32_m_100x32b.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_32b_prefixed() {
    const PATH: &str = "../samples/c32_m_100x32b.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_32b_check_prefixed() {
    const PATH: &str = "../samples/c32_m_100x32b.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_64b() {
    const PATH: &str = "../samples/c32_m_100x64b.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_multi_64b_alloc() {
    const PATH: &str = "../samples/c32_m_100x64b.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_64b_check() {
    const PATH: &str = "../samples/c32_m_100x64b.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_multi_64b_check_alloc() {
    const PATH: &str = "../samples/c32_m_100x64b.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_64b_prefixed() {
    const PATH: &str = "../samples/c32_m_100x64b.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_64b_check_prefixed() {
    const PATH: &str = "../samples/c32_m_100x64b.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_128b() {
    const PATH: &str = "../samples/c32_m_100x128b.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_multi_128b_alloc() {
    const PATH: &str = "../samples/c32_m_100x128b.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_128b_check() {
    const PATH: &str = "../samples/c32_m_100x128b.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_multi_128b_check_alloc() {
    const PATH: &str = "../samples/c32_m_100x128b.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_128b_prefixed() {
    const PATH: &str = "../samples/c32_m_100x128b.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_128b_check_prefixed() {
    const PATH: &str = "../samples/c32_m_100x128b.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_256b() {
    const PATH: &str = "../samples/c32_m_100x256b.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_multi_256b_alloc() {
    const PATH: &str = "../samples/c32_m_100x256b.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_256b_check() {
    const PATH: &str = "../samples/c32_m_100x256b.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_multi_256b_check_alloc() {
    const PATH: &str = "../samples/c32_m_100x256b.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_256b_prefixed() {
    const PATH: &str = "../samples/c32_m_100x256b.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_256b_check_prefixed() {
    const PATH: &str = "../samples/c32_m_100x256b.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_512b() {
    const PATH: &str = "../samples/c32_m_100x512b.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_multi_512b_alloc() {
    const PATH: &str = "../samples/c32_m_100x512b.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_512b_check() {
    const PATH: &str = "../samples/c32_m_100x512b.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_multi_512b_check_alloc() {
    const PATH: &str = "../samples/c32_m_100x512b.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_512b_prefixed() {
    const PATH: &str = "../samples/c32_m_100x512b.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_512b_check_prefixed() {
    const PATH: &str = "../samples/c32_m_100x512b.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_1k() {
    const PATH: &str = "../samples/c32_m_100x1k.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_multi_1k_alloc() {
    const PATH: &str = "../samples/c32_m_100x1k.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_1k_check() {
    const PATH: &str = "../samples/c32_m_100x1k.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_multi_1k_check_alloc() {
    const PATH: &str = "../samples/c32_m_100x1k.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_1k_prefixed() {
    const PATH: &str = "../samples/c32_m_100x1k.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_1k_check_prefixed() {
    const PATH: &str = "../samples/c32_m_100x1k.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_2k() {
    const PATH: &str = "../samples/c32_m_100x2k.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_multi_2k_alloc() {
    const PATH: &str = "../samples/c32_m_100x2k.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_2k_check() {
    const PATH: &str = "../samples/c32_m_100x2k.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_multi_2k_check_alloc() {
    const PATH: &str = "../samples/c32_m_100x2k.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_2k_prefixed() {
    const PATH: &str = "../samples/c32_m_100x2k.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_2k_check_prefixed() {
    const PATH: &str = "../samples/c32_m_100x2k.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_4k() {
    const PATH: &str = "../samples/c32_m_100x4k.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_multi_4k_alloc() {
    const PATH: &str = "../samples/c32_m_100x4k.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_4k_check() {
    const PATH: &str = "../samples/c32_m_100x4k.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_multi_4k_check_alloc() {
    const PATH: &str = "../samples/c32_m_100x4k.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_multi_4k_prefixed() {
    const PATH: &str = "../samples/c32_m_100x4k.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_multi_4k_check_prefixed() {
    const PATH: &str = "../samples/c32_m_100x4k.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_32k() {
    const PATH: &str = "../samples/c32_s_32k.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_single_32k_alloc() {
    const PATH: &str = "../samples/c32_s_32k.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_32k_check() {
    const PATH: &str = "../samples/c32_s_32k.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_single_32k_check_alloc() {
    const PATH: &str = "../samples/c32_s_32k.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_32k_prefixed() {
    const PATH: &str = "../samples/c32_s_32k.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_32k_check_prefixed() {
    const PATH: &str = "../samples/c32_s_32k.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_64k() {
    const PATH: &str = "../samples/c32_s_64k.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_single_64k_alloc() {
    const PATH: &str = "../samples/c32_s_64k.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_64k_check() {
    const PATH: &str = "../samples/c32_s_64k.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_single_64k_check_alloc() {
    const PATH: &str = "../samples/c32_s_64k.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_64k_prefixed() {
    const PATH: &str = "../samples/c32_s_64k.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_64k_check_prefixed() {
    const PATH: &str = "../samples/c32_s_64k.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_128k() {
    const PATH: &str = "../samples/c32_s_128k.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_single_128k_alloc() {
    const PATH: &str = "../samples/c32_s_128k.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_128k_check() {
    const PATH: &str = "../samples/c32_s_128k.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_single_128k_check_alloc() {
    const PATH: &str = "../samples/c32_s_128k.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_128k_prefixed() {
    const PATH: &str = "../samples/c32_s_128k.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_128k_check_prefixed() {
    const PATH: &str = "../samples/c32_s_128k.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_256k() {
    const PATH: &str = "../samples/c32_s_256k.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_single_256k_alloc() {
    const PATH: &str = "../samples/c32_s_256k.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_256k_check() {
    const PATH: &str = "../samples/c32_s_256k.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_single_256k_check_alloc() {
    const PATH: &str = "../samples/c32_s_256k.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_256k_prefixed() {
    const PATH: &str = "../samples/c32_s_256k.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_256k_check_prefixed() {
    const PATH: &str = "../samples/c32_s_256k.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_512k() {
    const PATH: &str = "../samples/c32_s_512k.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_single_512k_alloc() {
    const PATH: &str = "../samples/c32_s_512k.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_512k_check() {
    const PATH: &str = "../samples/c32_s_512k.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_single_512k_check_alloc() {
    const PATH: &str = "../samples/c32_s_512k.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_512k_prefixed() {
    const PATH: &str = "../samples/c32_s_512k.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_512k_check_prefixed() {
    const PATH: &str = "../samples/c32_s_512k.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_1m() {
    const PATH: &str = "../samples/c32_s_1m.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_single_1m_alloc() {
    const PATH: &str = "../samples/c32_s_1m.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_1m_check() {
    const PATH: &str = "../samples/c32_s_1m.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_single_1m_check_alloc() {
    const PATH: &str = "../samples/c32_s_1m.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_1m_prefixed() {
    const PATH: &str = "../samples/c32_s_1m.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_1m_check_prefixed() {
    const PATH: &str = "../samples/c32_s_1m.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_2m() {
    const PATH: &str = "../samples/c32_s_2m.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_single_2m_alloc() {
    const PATH: &str = "../samples/c32_s_2m.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_2m_check() {
    const PATH: &str = "../samples/c32_s_2m.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_single_2m_check_alloc() {
    const PATH: &str = "../samples/c32_s_2m.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_2m_prefixed() {
    const PATH: &str = "../samples/c32_s_2m.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_2m_check_prefixed() {
    const PATH: &str = "../samples/c32_s_2m.in";
    __internal::test_check_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_4m() {
    const PATH: &str = "../samples/c32_s_4m.in";
    __internal::test_default(PATH).unwrap();
}

#[test]
fn test_sample_single_4m_alloc() {
    const PATH: &str = "../samples/c32_s_4m.in";
    __internal::test_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_4m_check() {
    const PATH: &str = "../samples/c32_s_4m.in";
    __internal::test_check(PATH).unwrap();
}

#[test]
fn test_sample_single_4m_check_alloc() {
    const PATH: &str = "../samples/c32_s_4m.in";
    __internal::test_check_alloc(PATH).unwrap();
}

#[test]
fn test_sample_single_4m_prefixed() {
    const PATH: &str = "../samples/c32_s_4m.in";
    __internal::test_prefixed(PATH).unwrap();
}

#[test]
fn test_sample_single_4m_check_prefixed() {
    const PATH: &str = "../samples/c32_s_4m.in";
    __internal::test_check_prefixed(PATH).unwrap();
}
