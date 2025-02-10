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
   test_c32_samples,
   fn(path: &str) {
       let bytes = std::fs::read(path).unwrap();

       let mut ebuffer = vec![0; c32::encoded_len(bytes.len())];
       let pos = c32::encode_into(&bytes, &mut ebuffer).unwrap();

       let mut dbuffer = vec![0; c32::decoded_len(pos)];
       let pos = c32::decode_into(&ebuffer[..pos], &mut dbuffer).unwrap();

       assert_eq!(&dbuffer[..pos], &bytes);
   },
   "multi_32b": ("samples/c32_m_100x32b.in"),
   "multi_64b": ("samples/c32_m_100x64b.in"),
   "multi_128b": ("samples/c32_m_100x128b.in"),
   "multi_256b": ("samples/c32_m_100x256b.in"),
   "multi_512b": ("samples/c32_m_100x512b.in"),
   "multi_1k": ("samples/c32_m_100x1k.in"),
   "multi_2k": ("samples/c32_m_100x2k.in"),
   "multi_4k": ("samples/c32_m_100x4k.in"),
   "single_32k": ("samples/c32_s_32k.in"),
   "single_64k": ("samples/c32_s_64k.in"),
   "single_128k": ("samples/c32_s_128k.in"),
   "single_256k": ("samples/c32_s_256k.in"),
   "single_512k": ("samples/c32_s_512k.in"),
   "single_1m": ("samples/c32_s_1m.in"),
   "single_2m": ("samples/c32_s_2m.in"),
   "single_4m": ("samples/c32_s_4m.in")
}

#[cfg(feature = "alloc")]
test_cases! {
   test_c32_samples_alloc,
   fn(path: &str) {
       let bytes = std::fs::read(path).unwrap();

       let encoded = c32::encode(&bytes);
       let decoded = c32::decode(encoded).unwrap();

       assert_eq!(decoded, bytes);
   },
   "multi_32b": ("samples/c32_m_100x32b.in"),
   "multi_64b": ("samples/c32_m_100x64b.in"),
   "multi_128b": ("samples/c32_m_100x128b.in"),
   "multi_256b": ("samples/c32_m_100x256b.in"),
   "multi_512b": ("samples/c32_m_100x512b.in"),
   "multi_1k": ("samples/c32_m_100x1k.in"),
   "multi_2k": ("samples/c32_m_100x2k.in"),
   "multi_4k": ("samples/c32_m_100x4k.in"),
   "single_32k": ("samples/c32_s_32k.in"),
   "single_64k": ("samples/c32_s_64k.in"),
   "single_128k": ("samples/c32_s_128k.in"),
   "single_256k": ("samples/c32_s_256k.in"),
   "single_512k": ("samples/c32_s_512k.in"),
   "single_1m": ("samples/c32_s_1m.in"),
   "single_2m": ("samples/c32_s_2m.in"),
   "single_4m": ("samples/c32_s_4m.in")
}

#[cfg(feature = "check")]
test_cases! {
    test_c32_check_samples,
    fn(path: &str) {
        let bytes = std::fs::read(path).unwrap();
        let version = 22;

        let mut ebuffer = vec![0; c32::encoded_check_len(bytes.len())];
        let pos = c32::encode_check_into(&bytes, version, &mut ebuffer).unwrap();

        let mut dbuffer = vec![0; c32::decoded_check_len(pos)];
        let (ver, pos) = c32::decode_check_into(&ebuffer[..pos], &mut dbuffer).unwrap();

        assert_eq!(&dbuffer[..pos], &bytes);
        assert_eq!(ver, version);
    },
    "multi_32b": ("samples/c32_m_100x32b.in"),
    "multi_64b": ("samples/c32_m_100x64b.in"),
    "multi_128b": ("samples/c32_m_100x128b.in"),
    "multi_256b": ("samples/c32_m_100x256b.in"),
    "multi_512b": ("samples/c32_m_100x512b.in"),
    "multi_1k": ("samples/c32_m_100x1k.in"),
    "multi_2k": ("samples/c32_m_100x2k.in"),
    "multi_4k": ("samples/c32_m_100x4k.in"),
    "single_32k": ("samples/c32_s_32k.in"),
    "single_64k": ("samples/c32_s_64k.in"),
    "single_128k": ("samples/c32_s_128k.in"),
    "single_256k": ("samples/c32_s_256k.in"),
    "single_512k": ("samples/c32_s_512k.in"),
    "single_1m": ("samples/c32_s_1m.in"),
    "single_2m": ("samples/c32_s_2m.in"),
    "single_4m": ("samples/c32_s_4m.in")
}

#[cfg(all(feature = "alloc", feature = "check"))]
test_cases! {
    test_c32_check_samples_alloc,
    fn(path: &str) {
        let bytes = std::fs::read(path).unwrap();
        let version = 22;

        let encoded = c32::encode_check(&bytes, version).unwrap();
        let (ver, decoded) = c32::decode_check(&encoded).unwrap();

        assert_eq!(decoded, bytes);
        assert_eq!(ver, version);
    },
    "multi_32b": ("samples/c32_m_100x32b.in"),
    "multi_64b": ("samples/c32_m_100x64b.in"),
    "multi_128b": ("samples/c32_m_100x128b.in"),
    "multi_256b": ("samples/c32_m_100x256b.in"),
    "multi_512b": ("samples/c32_m_100x512b.in"),
    "multi_1k": ("samples/c32_m_100x1k.in"),
    "multi_2k": ("samples/c32_m_100x2k.in"),
    "multi_4k": ("samples/c32_m_100x4k.in"),
    "single_32k": ("samples/c32_s_32k.in"),
    "single_64k": ("samples/c32_s_64k.in"),
    "single_128k": ("samples/c32_s_128k.in"),
    "single_256k": ("samples/c32_s_256k.in"),
    "single_512k": ("samples/c32_s_512k.in"),
    "single_1m": ("samples/c32_s_1m.in"),
    "single_2m": ("samples/c32_s_2m.in"),
    "single_4m": ("samples/c32_s_4m.in")
}
