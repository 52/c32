#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|bytes: &[u8]| {
    // Skip empty bytes.
    if bytes.is_empty() {
        return;
    }

    // Skip zero bytes.
    if bytes.iter().all(|b| *b == 0) {
        return;
    }

    // Get a prefix character and version from the input.
    let prefix = (bytes[0] % 26 + b'A') as char;
    let version = bytes[0] % 32;

    // Fuzzes the #[feature = "alloc"] paths.
    {
        let en = c32::encode(bytes);
        let de = c32::decode(&en).unwrap();
        assert_eq!(de, bytes);
    }

    // Fuzzes the prefixed #[feature = "alloc"] paths.
    {
        let en = c32::encode_prefixed(bytes, prefix);
        let de = c32::decode_prefixed(&en, prefix).unwrap();
        assert_eq!(de, bytes);
    }

    // Fuzzes the #[feature = "alloc"] + #[feature = "check"] paths.
    {
        if let Ok(en) = c32::encode_check(bytes, version) {
            let (de, dv) = c32::decode_check(&en).unwrap();
            assert_eq!(de, bytes);
            assert_eq!(dv, version);
        }
    }

    // Fuzzes the prefixed #[feature = "alloc"] + #[feature = "check"] paths.
    {
        if let Ok(en) = c32::encode_check_prefixed(bytes, prefix, version) {
            let (de, dv) = c32::decode_check_prefixed(&en, prefix).unwrap();
            assert_eq!(de, bytes);
            assert_eq!(dv, version);
        }
    }
});
