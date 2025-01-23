// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#[macro_export]
macro_rules! gen_c32_samples_test {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            let input = std::fs::read($path).unwrap();

            let encoded = c32::encode(&input);
            let decoded = c32::decode(&encoded).unwrap();
            assert_eq!(decoded, input);

            let encoded_roundtrip = c32::encode(&decoded);
            assert_eq!(encoded, encoded_roundtrip);
        }
    };
}

#[macro_export]
macro_rules! gen_c32_rand_test {
    ($name:ident, $iterations:expr, $max_len:expr) => {
        #[test]
        fn $name() {
            use rand::distr::Alphanumeric;
            use rand::distr::SampleString;
            use rand::Rng;

            let mut rng = rand::rng();

            for _ in 0..$iterations {
                let len = rng.random_range(0..=$max_len);
                let input = Alphanumeric.sample_string(&mut rand::rng(), len);

                let encoded = c32::encode(&input);
                let decoded = c32::decode(&encoded).unwrap();
                let encoded_roundtrip = c32::encode(&decoded);

                assert_eq!(String::from_utf8(decoded).unwrap(), input);
                assert_eq!(encoded, encoded_roundtrip);
            }
        }
    };
}

#[macro_export]
macro_rules! gen_c32_unit_test {
    ($name:ident, $bytes:expr, $str:expr) => {
        #[test]
        fn $name() {
            let bytes = $bytes;
            let encoded = c32::encode(&bytes);
            assert_eq!(encoded, $str);

            let decoded = c32::decode($str).unwrap();
            assert_eq!(decoded, bytes);

            let encoded_roundtrip = c32::encode(&decoded);
            assert_eq!(encoded, encoded_roundtrip)
        }
    };
}

#[macro_export]
macro_rules! gen_c32_error_test {
    ($name:ident, $input:expr, $error:expr) => {
        #[test]
        fn $name() {
            assert_eq!(c32::decode($input).unwrap_err(), $error);
        }
    };
}
