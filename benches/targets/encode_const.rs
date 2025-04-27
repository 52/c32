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

/// A benchmark for default encoding functions.
fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_const");

    macro_rules! bench {
        ($name:expr, $n:expr, $sample:expr) => {
            group.bench_function(f!("encode_const_{}", $name), |b| {
                b.iter(|| c32::Buffer::<$n>::encode(black_box($sample)));
            });
        };
    }

    bench!("m_100x32b", 5280, samples::M_100X32B);
    bench!("m_100x64b", 10400, samples::M_100X64B);
    bench!("m_100x128b", 20640, samples::M_100X128B);
    bench!("m_100x256b", 41120, samples::M_100X256B);
    bench!("m_100x512b", 82080, samples::M_100X512B);

    group.finish();
}

/// A benchmark for checksum encoding functions.
fn bench_encode_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_check_const");

    macro_rules! bench {
        ($name:expr, $n:expr, $sample:expr) => {
            group.bench_function(f!("encode_check_const_{}", $name), |b| {
                b.iter(|| c32::Buffer::<$n>::encode_check(black_box($sample), 0));
            });
        };
    }

    bench!("m_100x32b", 5288, samples::M_100X32B);
    bench!("m_100x64b", 10408, samples::M_100X64B);
    bench!("m_100x128b", 20648, samples::M_100X128B);
    bench!("m_100x256b", 41128, samples::M_100X256B);
    bench!("m_100x512b", 82088, samples::M_100X512B);

    group.finish();
}

/// A benchmark for prefixed encoding functions.
fn bench_encode_prefixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_prefixed_const");

    macro_rules! bench {
        ($name:expr, $n:expr, $sample:expr) => {
            group.bench_function(f!("encode_prefixed_const_{}", $name), |b| {
                b.iter(|| c32::Buffer::<$n>::encode_prefixed(black_box($sample), 'S'));
            });
        };
    }

    bench!("m_100x32b", 5281, samples::M_100X32B);
    bench!("m_100x64b", 10401, samples::M_100X64B);
    bench!("m_100x128b", 20641, samples::M_100X128B);
    bench!("m_100x256b", 41121, samples::M_100X256B);
    bench!("m_100x512b", 82081, samples::M_100X512B);

    group.finish();
}

/// A benchmark for prefixed checksum encoding functions.
fn bench_encode_check_prefixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_check_prefixed_const");

    macro_rules! bench {
        ($name:expr, $n:expr, $sample:expr) => {
            group.bench_function(f!("encode_check_prefixed_const_{}", $name), |b| {
                b.iter(|| c32::Buffer::<$n>::encode_check_prefixed(black_box($sample), 'S', 0));
            });
        };
    }

    bench!("m_100x32b", 5289, samples::M_100X32B);
    bench!("m_100x64b", 10409, samples::M_100X64B);
    bench!("m_100x128b", 20649, samples::M_100X128B);
    bench!("m_100x256b", 41129, samples::M_100X256B);
    bench!("m_100x512b", 82089, samples::M_100X512B);

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
