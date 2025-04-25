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
use rand::distr::Alphanumeric;
use rand::distr::SampleString;
use rand::Rng;

mod __internal {
    use super::*;

    /// A test helper for default features.
    pub fn test_default(len: usize, rounds: usize) -> Result<()> {
        let mut rng = rand::rng();
        for _ in 0..rounds {
            let len = rng.random_range(0..=len);
            let input = Alphanumeric.sample_string(&mut rng, len);

            let mut ebuf = vec![0u8; encoded_len(input.len())];
            let epos = encode_into(input.as_bytes(), &mut ebuf)?;

            let mut dbuf = vec![0u8; decoded_len(epos)];
            let dpos = decode_into(&ebuf[..epos], &mut dbuf)?;
            assert_eq!(&dbuf[..dpos], input.as_bytes());
        }
        Ok(())
    }

    /// A test helper for `[feature = "alloc"]`.
    pub fn test_alloc(len: usize, rounds: usize) -> Result<()> {
        let mut rng = rand::rng();
        for _ in 0..rounds {
            let len = rng.random_range(0..=len);
            let input = Alphanumeric.sample_string(&mut rng, len);

            let en = encode(&input);
            let de = decode(&en)?;
            assert_eq!(de, input.as_bytes());
        }
        Ok(())
    }

    /// A test helper for `[feature = "check"]`.
    pub fn test_check(len: usize, rounds: usize) -> Result<()> {
        let mut rng = rand::rng();
        for _ in 0..rounds {
            let len = rng.random_range(0..=len);
            let input = Alphanumeric.sample_string(&mut rng, len);
            let version = rng.random_range(0..32);

            let mut ebuf = vec![0u8; encoded_check_len(input.len())];
            let epos = encode_check_into(input.as_bytes(), &mut ebuf, version)?;

            let mut dbuf = vec![0u8; decoded_check_len(epos)];
            let (dpos, dver) = decode_check_into(&ebuf[..epos], &mut dbuf)?;

            assert_eq!(&dbuf[..dpos], input.as_bytes());
            assert_eq!(dver, version);
        }
        Ok(())
    }

    /// A test helper for `[feature = "check"]` + `[feature = "alloc"]`.
    pub fn test_check_alloc(len: usize, rounds: usize) -> Result<()> {
        let mut rng = rand::rng();
        for _ in 0..rounds {
            let len = rng.random_range(0..=len);
            let input = Alphanumeric.sample_string(&mut rng, len);
            let version = rng.random_range(0..32);

            let en = encode_check(&input, version)?;
            let (de, de_version) = decode_check(&en)?;

            assert_eq!(de, input.as_bytes());
            assert_eq!(de_version, version);
        }
        Ok(())
    }

    /// A test helper for prefixed encoding/decoding.
    pub fn test_prefixed(len: usize, rounds: usize) -> Result<()> {
        let mut rng = rand::rng();
        for _ in 0..rounds {
            let len = rng.random_range(0..=len);
            let input = Alphanumeric.sample_string(&mut rng, len);
            let prefix = rng.random::<char>();

            let en = encode_prefixed(&input, prefix);
            let de = decode_prefixed(&en, prefix)?;

            assert_eq!(de, input.as_bytes());
        }
        Ok(())
    }

    /// A test helper for `[feature = "check"]` prefixed encoding/decoding.
    pub fn test_check_prefixed(len: usize, rounds: usize) -> Result<()> {
        let mut rng = rand::rng();
        for _ in 0..rounds {
            let len = rng.random_range(0..=len);
            let input = Alphanumeric.sample_string(&mut rng, len);
            let version = rng.random_range(0..32);
            let prefix = rng.random::<char>();

            let en = encode_check_prefixed(&input, prefix, version)?;
            let (de, de_version) = decode_check_prefixed(&en, prefix)?;

            assert_eq!(de, input.as_bytes());
            assert_eq!(de_version, version);
        }
        Ok(())
    }
}

#[test]
fn test_rand_xs() {
    __internal::test_default(1, 10_000).unwrap()
}

#[test]
fn test_rand_xs_alloc() {
    __internal::test_alloc(1, 10_000).unwrap()
}

#[test]
fn test_rand_xs_check() {
    __internal::test_check(1, 10_000).unwrap()
}

#[test]
fn test_rand_xs_check_alloc() {
    __internal::test_check_alloc(1, 10_000).unwrap()
}

#[test]
fn test_rand_xs_prefixed() {
    __internal::test_prefixed(1, 10_000).unwrap()
}

#[test]
fn test_rand_xs_check_prefixed() {
    __internal::test_check_prefixed(1, 10_000).unwrap()
}

#[test]
fn test_rand_sm() {
    __internal::test_default(10, 10_000).unwrap()
}

#[test]
fn test_rand_sm_alloc() {
    __internal::test_alloc(10, 10_000).unwrap()
}

#[test]
fn test_rand_sm_check() {
    __internal::test_check(10, 10_000).unwrap()
}

#[test]
fn test_rand_sm_check_alloc() {
    __internal::test_check_alloc(10, 10_000).unwrap()
}

#[test]
fn test_rand_sm_prefixed() {
    __internal::test_prefixed(10, 10_000).unwrap()
}

#[test]
fn test_rand_sm_check_prefixed() {
    __internal::test_check_prefixed(10, 10_000).unwrap()
}

#[test]
fn test_rand_lg() {
    __internal::test_default(1_000, 1_000).unwrap()
}

#[test]
fn test_rand_lg_alloc() {
    __internal::test_alloc(1_000, 1_000).unwrap()
}

#[test]
fn test_rand_lg_check() {
    __internal::test_check(1_000, 1_000).unwrap()
}

#[test]
fn test_rand_lg_check_alloc() {
    __internal::test_check_alloc(1_000, 1_000).unwrap()
}

#[test]
fn test_rand_lg_prefixed() {
    __internal::test_prefixed(1_000, 1_000).unwrap()
}

#[test]
fn test_rand_lg_check_prefixed() {
    __internal::test_check_prefixed(1_000, 1_000).unwrap()
}

#[test]
fn test_rand_xl() {
    __internal::test_default(10_000, 1_000).unwrap()
}

#[test]
fn test_rand_xl_alloc() {
    __internal::test_alloc(10_000, 1_000).unwrap()
}

#[test]
fn test_rand_xl_check() {
    __internal::test_check(10_000, 1_000).unwrap()
}

#[test]
fn test_rand_xl_check_alloc() {
    __internal::test_check_alloc(10_000, 1_000).unwrap()
}

#[test]
fn test_rand_xl_prefixed() {
    __internal::test_prefixed(10_000, 1_000).unwrap()
}

#[test]
fn test_rand_xl_check_prefixed() {
    __internal::test_check_prefixed(10_000, 1_000).unwrap()
}
