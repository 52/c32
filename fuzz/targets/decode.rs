#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|bytes: &[u8]| {
    let encoded = c32::encode(bytes);
    let decoded = c32::decode(encoded).unwrap();
    assert_eq!(decoded, bytes);
});
