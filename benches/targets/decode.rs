// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use format as f;

mod samples;

/// A benchmark for default decoding functions.
fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");

    for (sample, bytes) in samples::ALL {
        let en = c32::encode(bytes);
        let en_bytes = en.as_bytes();

        // [`c32::decode_into`]
        group.bench_function(f!("decode_into_{sample}"), |b| {
            let capacity = c32::decoded_len(en_bytes.len());
            let mut dst = vec![0u8; capacity];
            b.iter(|| c32::decode_into(black_box(en_bytes), &mut dst).unwrap());
        });

        // [`c32::decode`]
        group.bench_function(f!("decode_{sample}"), |b| {
            b.iter(|| c32::decode(black_box(&en)).unwrap());
        });
    }

    group.finish();
}

/// A benchmark for checksum decoding functions.
fn bench_decode_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_check");

    for (sample, bytes) in samples::ALL {
        let en = c32::encode_check(bytes, 0).unwrap();
        let en_bytes = en.as_bytes();

        // [`c32::decode_check_into`]
        group.bench_function(f!("decode_check_into_{sample}"), |b| {
            let capacity = c32::decoded_check_len(en_bytes.len());
            let mut dst = vec![0u8; capacity];
            b.iter(|| c32::decode_check_into(black_box(en_bytes), &mut dst).unwrap());
        });

        // [`c32::decode_check`]
        group.bench_function(f!("decode_check_{sample}"), |b| {
            b.iter(|| c32::decode_check(black_box(&en)).unwrap());
        });
    }

    group.finish();
}

/// A benchmark for prefixed decoding functions.
fn bench_decode_prefixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_prefixed");

    for (sample, bytes) in samples::ALL {
        let en = c32::encode_prefixed(bytes, 'S');

        // [`c32::decode_prefixed`]
        group.bench_function(f!("decode_prefixed_{sample}"), |b| {
            b.iter(|| c32::decode_prefixed(black_box(&en), 'S').unwrap());
        });
    }

    group.finish();
}

/// A benchmark for prefixed checksum decoding functions.
fn bench_decode_check_prefixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_check_prefixed");

    for (sample, bytes) in samples::ALL {
        let en = c32::encode_check_prefixed(bytes, 'S', 0).unwrap();

        // [`c32::decode_check_prefixed`]
        group.bench_function(f!("decode_check_prefixed_{sample}"), |b| {
            b.iter(|| c32::decode_check_prefixed(black_box(&en), 'S').unwrap());
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
