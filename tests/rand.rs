// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use rand::distr::Alphanumeric;
use rand::distr::SampleString;
use rand::Rng;

#[path = "macro.rs"]
mod macros;

test_cases! {
    test_c32_rand,
    fn(range: usize, len: usize) {
        let mut rng = rand::rng();

        for _ in 0..range {
            let len = rng.random_range(0..=len);
            let input = Alphanumeric.sample_string(&mut rng, len);

            let mut ebuffer = vec![0; c32::encoded_len(input.len())];
            let pos = c32::encode_into(input.as_bytes(), &mut ebuffer).unwrap();

            let mut dbuffer = vec![0; c32::decoded_len(pos)];
            let pos = c32::decode_into(&ebuffer[..pos], &mut dbuffer).unwrap();

            assert_eq!(&dbuffer[..pos], input.as_bytes());
        }
    },
    "xs": (10_000, 1),
    "sm": (10_000, 10),
    "lg": (1_000, 1_000),
    "xl": (1_000, 10_000),
}

#[cfg(feature = "alloc")]
test_cases! {
    test_c32_rand_alloc,
    fn(range: usize, len: usize) {
        let mut rng = rand::rng();

        for _ in 0..range {
            let len = rng.random_range(0..=len);
            let input = Alphanumeric.sample_string(&mut rng, len);

            let encoded = c32::encode(&input);
            let decoded = c32::decode(encoded).unwrap();

            assert_eq!(decoded, input.as_bytes());
        }
    },
    "xs": (10_000, 1),
    "sm": (10_000, 10),
    "lg": (1_000, 1_000),
    "xl": (1_000, 10_000),
}

#[cfg(feature = "check")]
test_cases! {
    test_c32_check_rand,
    fn(range: usize, len: usize) {
        let mut rng = rand::rng();

        for _ in 0..range {
            let len = rng.random_range(0..=len);
            let version = rng.random_range(u8::MIN..32);
            let input = Alphanumeric.sample_string(&mut rng, len);

            let mut ebuffer = vec![0; c32::encoded_check_len(input.len())];
            let pos = c32::encode_check_into(input.as_bytes(), version, &mut ebuffer).unwrap();

            let mut dbuffer = vec![0; c32::decoded_check_len(pos)];
            let (ver, pos) = c32::decode_check_into(&ebuffer[..pos],&mut dbuffer).unwrap();

            assert_eq!(&dbuffer[..pos], input.as_bytes());
            assert_eq!(ver, version)
        }
    },
    "xs": (10_000, 1),
    "sm": (10_000, 10),
    "lg": (1_000, 1_000),
    "xl": (1_000, 10_000),
}

#[cfg(all(feature = "alloc", feature = "check"))]
test_cases! {
    test_c32_check_rand_alloc,
    fn(range: usize, len: usize) {
        let mut rng = rand::rng();

        for _ in 0..range {
            let len = rng.random_range(0..=len);
            let version = rng.random_range(u8::MIN..32);
            let input = Alphanumeric.sample_string(&mut rng, len);

            let encoded = c32::encode_check(input.as_bytes(), version).unwrap();
            let (ver, decoded) = c32::decode_check(&encoded).unwrap();

            assert_eq!(decoded, input.as_bytes());
            assert_eq!(ver, version);
        }
    },
    "xs": (10_000, 1),
    "sm": (10_000, 10),
    "lg": (1_000, 1_000),
    "xl": (1_000, 10_000),
}
