// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use test_case::test_case;

#[test_case(10_000, 10;   "encode rand small")]
#[test_case(100, 1_000;   "encode rand large")]
#[test_case(1_000, 100;   "encode rand mid")]
fn test_c32_rand(iter: usize, len: usize) {
    use rand::distr::Alphanumeric;
    use rand::distr::SampleString;
    use rand::Rng;

    let mut rng = rand::rng();

    for _ in 0..iter {
        let len = rng.random_range(0..=len);
        let input = Alphanumeric.sample_string(&mut rng, len);

        let mut ebuffer = vec![0; c32::encoded_len(input.len())];
        let pos = c32::encode_into(input.as_bytes(), &mut ebuffer).unwrap();

        let mut dbuffer = vec![0; c32::decoded_len(pos)];
        let pos = c32::decode_into(&ebuffer[..pos], &mut dbuffer).unwrap();

        assert_eq!(&dbuffer[..pos], input.as_bytes());
    }
}
