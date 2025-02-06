// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#[macro_export]
macro_rules! c32_samples_test {
    ($prefix:ident, $path:expr) => {
        paste::paste! {
            #[test]
            #[cfg(feature = "alloc")]
            fn [<$prefix _alloc>]() {
                let input = std::fs::read($path).unwrap();

                let encoded = {
                    let capacity = c32::encoded_len(input.len());
                    let mut output = vec![0; capacity];

                    let written = c32::encode_into(&input, &mut output).unwrap();
                    assert!(written <= capacity);
                    output.truncate(written);

                    String::from_utf8(output).unwrap()
                };

                let decoded = {
                    let capacity = c32::decoded_len(encoded.len());
                    let mut output = vec![0; capacity];

                    let written = c32::decode_into(&encoded, &mut output).unwrap();
                    assert!(written <= capacity);
                    output.truncate(written);

                    output
                };

                let encoded_roundtrip = c32::encode(&decoded);

                assert_eq!(decoded, input);
                assert_eq!(encoded, encoded_roundtrip);
            }

            #[test]
            #[cfg(feature = "std")]
            fn [<$prefix _std>]() {
                let input = std::fs::read($path).unwrap();

                let encoded = c32::encode(&input);
                let decoded = c32::decode(&encoded).unwrap();
                let encoded_roundtrip = c32::encode(&decoded);

                assert_eq!(decoded, input);
                assert_eq!(encoded, encoded_roundtrip);
            }
        }
    };
}

#[macro_export]
macro_rules! c32_rand_test {
    ($prefix:ident, $iterations:expr, $max_len:expr) => {
        paste::paste!{
            #[test]
            #[cfg(feature = "alloc")]
            fn [<$prefix _alloc>]() {
                use rand::distr::Alphanumeric;
                use rand::distr::SampleString;
                use rand::Rng;

                let mut rng = rand::rng();

                for _ in 0..$iterations {
                    let len = rng.random_range(0..=$max_len);
                    let input = Alphanumeric.sample_string(&mut rand::rng(), len);

                    let encoded = {
                        let capacity = c32::encoded_len(input.len());
                        let mut output = vec![0; capacity];

                        let written = c32::encode_into(&input, &mut output).unwrap();
                        assert!(written <= capacity);
                        output.truncate(written);

                        String::from_utf8(output).unwrap()
                    };

                    let decoded = {
                        let capacity = c32::decoded_len(encoded.len());
                        let mut output = vec![0; capacity];

                        let written = c32::decode_into(&encoded, &mut output).unwrap();
                        assert!(written <= capacity);
                        output.truncate(written);

                        output
                    };

                    let encoded_roundtrip = c32::encode(&decoded);

                    assert_eq!(decoded, input.as_bytes());
                    assert_eq!(encoded, encoded_roundtrip);
                }
            }

            #[test]
            #[cfg(feature = "std")]
            fn [<$prefix _std>]() {
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

                    assert_eq!(decoded, input.as_bytes());
                    assert_eq!(encoded, encoded_roundtrip);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! c32_unit_test {
    ($prefix:ident, $input:expr, $str:expr) => {
        paste::paste! {
            #[test]
            #[cfg(feature = "alloc")]
            fn [<$prefix _alloc>]() {
                let input = $input;

                let encoded = {
                    let capacity = c32::encoded_len(input.len());
                    let mut output = vec![0; capacity];

                    let written = c32::encode_into(&input, &mut output).unwrap();
                    assert!(written <= capacity);
                    output.truncate(written);

                    String::from_utf8(output).unwrap()
                };

                let decoded = {
                    let capacity = c32::decoded_len(encoded.len());
                    let mut output = vec![0; capacity];

                    let written = c32::decode_into(&encoded, &mut output).unwrap();
                    assert!(written <= capacity);
                    output.truncate(written);

                    output
                };

                let encoded_roundtrip = c32::encode(&decoded);

                assert_eq!(encoded, $str);
                assert_eq!(decoded, input);
                assert_eq!(encoded, encoded_roundtrip);
            }

            #[test]
            #[cfg(feature = "std")]
            fn [<$prefix _std>]() {
                let input = $input;

                let encoded = c32::encode(&input);
                let decoded = c32::decode(&encoded).unwrap();
                let encoded_roundtrip = c32::encode(&decoded);

                assert_eq!(encoded, $str);
                assert_eq!(decoded, input);
                assert_eq!(encoded, encoded_roundtrip);
            }
        }
    };
}

#[macro_export]
macro_rules! c32_error_test {
    ($prefix:ident, $input:expr, $error:expr) => {
        paste::paste! {
            #[test]
            #[cfg(feature = "alloc")]
            fn [<$prefix _alloc>]() {
                let input = $input;
                let capacity = c32::decoded_len(input.len());
                let mut output = vec![0; capacity];
                assert_eq!(c32::decode_into(input, &mut output).unwrap_err(), $error);
            }

            #[test]
            #[cfg(feature = "std")]
            fn [<$prefix _std>]() {
                let input = $input;
                assert_eq!(c32::decode(input).unwrap_err(), $error);
            }
        }
    };
}
