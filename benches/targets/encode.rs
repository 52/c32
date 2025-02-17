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

/// A benchmark for default encoding functions.
fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");

    for sample in SAMPLES {
        let bytes = read(f!("../samples/c32_{sample}.in")).unwrap();

        // [`c32::encode_into`]
        group.bench_function(f!("encode_into_{sample}"), |b| {
            let capacity = c32::encoded_len(bytes.len());
            let mut dst = vec![0u8; capacity];
            b.iter(|| c32::encode_into(&bytes, &mut dst).unwrap());
        });

        // [`c32::encode`]
        group.bench_function(f!("encode_{sample}"), |b| {
            b.iter(|| c32::encode(&bytes));
        });
    }

    group.finish();
}

/// A benchmark for checksum encoding functions.
fn bench_encode_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_check");

    for sample in SAMPLES {
        let bytes = read(f!("../samples/c32_{sample}.in")).unwrap();

        // [`c32::encode_check_into`]
        group.bench_function(f!("encode_check_into_{sample}"), |b| {
            let capacity = c32::encoded_check_len(bytes.len());
            let mut dst = vec![0u8; capacity];
            b.iter(|| c32::encode_check_into(&bytes, &mut dst, 0).unwrap());
        });

        // [`c32::encode_check`]
        group.bench_function(f!("encode_check_{sample}"), |b| {
            b.iter(|| c32::encode_check(&bytes, 0).unwrap());
        });
    }

    group.finish();
}

/// A benchmark for prefixed encoding functions.
fn bench_encode_prefixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_prefixed");

    for sample in SAMPLES {
        let bytes = read(f!("../samples/c32_{sample}.in")).unwrap();

        // [`c32::encode_prefixed`]
        group.bench_function(f!("encode_prefixed_{sample}"), |b| {
            b.iter(|| c32::encode_prefixed(&bytes, 'S'));
        });
    }

    group.finish();
}

/// A benchmark for prefixed checksum encoding functions.
fn bench_encode_check_prefixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_check_prefixed");

    for sample in SAMPLES {
        let bytes = read(f!("../samples/c32_{sample}.in")).unwrap();

        // [`c32::encode_check_prefixed`]
        group.bench_function(f!("encode_check_prefixed_{sample}"), |b| {
            b.iter(|| c32::encode_check_prefixed(&bytes, 'S', 0).unwrap());
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_encode,
    bench_encode_check,
    bench_encode_prefixed,
    bench_encode_check_prefixed
);

criterion_main!(benches);
