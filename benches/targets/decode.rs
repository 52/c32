// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::fs::read;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use format as f;

const SAMPLES: [&str; 16] = [
    "m_100x32b",
    "m_100x64b",
    "m_100x128b",
    "m_100x256b",
    "m_100x512b",
    "m_100x1k",
    "m_100x2k",
    "m_100x4k",
    "s_32k",
    "s_64k",
    "s_128k",
    "s_256k",
    "s_512k",
    "s_1m",
    "s_2m",
    "s_4m",
];

/// A benchmark for default decoding functions.
fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");

    for sample in SAMPLES {
        let bytes = read(f!("../samples/c32_{sample}.in")).unwrap();
        let en = c32::encode(bytes);
        let en_bytes = en.as_bytes();

        // [`c32::decode_into`]
        group.bench_function(f!("decode_into_{sample}"), |b| {
            let capacity = c32::decoded_len(en_bytes.len());
            let mut dst = vec![0u8; capacity];
            b.iter(|| c32::decode_into(en_bytes, &mut dst).unwrap());
        });

        // [`c32::decode`]
        group.bench_function(f!("decode_{sample}"), |b| {
            b.iter(|| c32::decode(&en).unwrap());
        });
    }

    group.finish();
}

/// A benchmark for checksum decoding functions.
fn bench_decode_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_check");

    for sample in SAMPLES {
        let bytes = read(f!("../samples/c32_{sample}.in")).unwrap();
        let en = c32::encode_check(bytes, 0).unwrap();
        let en_bytes = en.as_bytes();

        // [`c32::decode_check_into`]
        group.bench_function(f!("decode_check_into_{sample}"), |b| {
            let capacity = c32::decoded_check_len(en_bytes.len());
            let mut dst = vec![0u8; capacity];
            b.iter(|| c32::decode_check_into(en_bytes, &mut dst).unwrap());
        });

        // [`c32::decode_check`]
        group.bench_function(f!("decode_check_{sample}"), |b| {
            b.iter(|| c32::decode_check(&en).unwrap());
        });
    }

    group.finish();
}

/// A benchmark for prefixed decoding functions.
fn bench_decode_prefixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_prefixed");

    for sample in SAMPLES {
        let bytes = read(f!("../samples/c32_{sample}.in")).unwrap();
        let en = c32::encode_prefixed(bytes, 'S');

        // [`c32::decode_prefixed`]
        group.bench_function(f!("decode_prefixed_{sample}"), |b| {
            b.iter(|| c32::decode_prefixed(&en, 'S').unwrap());
        });
    }

    group.finish();
}

/// A benchmark for prefixed checksum decoding functions.
fn bench_decode_check_prefixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_check_prefixed");

    for sample in SAMPLES {
        let bytes = read(f!("../samples/c32_{sample}.in")).unwrap();
        let en = c32::encode_check_prefixed(bytes, 'S', 0).unwrap();

        // [`c32::decode_check_prefixed`]
        group.bench_function(f!("decode_check_prefixed_{sample}"), |b| {
            b.iter(|| c32::decode_check_prefixed(&en, 'S').unwrap());
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_decode,
    bench_decode_check,
    bench_decode_prefixed,
    bench_decode_check_prefixed
);

criterion_main!(benches);
