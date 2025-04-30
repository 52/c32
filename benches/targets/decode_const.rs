// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use c32::en::Check;
use c32::Buffer;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use format as f;

mod samples;

/// A benchmark for default decoding functions.
fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_const");

    macro_rules! bench {
        ($name:expr, $n:expr, $sample:expr) => {
            let en = c32::encode(*$sample);
            let en_bytes = en.as_bytes();
            group.bench_function(f!("decode_const_{}", $name), |b| {
                b.iter(|| Buffer::<$n>::decode(black_box(en_bytes)));
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

/// A benchmark for checksum decoding functions.
fn bench_decode_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_check_const");

    macro_rules! bench {
        ($name:expr, $n:expr, $sample:expr) => {
            let en = c32::encode_check(*$sample, 0).unwrap();
            let en_bytes = en.as_bytes();
            group.bench_function(f!("decode_check_const_{}", $name), |b| {
                b.iter(|| {
                    Buffer::<$n, false, Check>::decode(black_box(en_bytes))
                });
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

/// A benchmark for prefixed decoding functions.
fn bench_decode_prefixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_prefixed_const");

    macro_rules! bench {
        ($name:expr, $n:expr, $sample:expr) => {
            let en = c32::encode_prefixed(*$sample, 'S');
            let en_bytes = en.as_bytes();
            group.bench_function(f!("decode_prefixed_const_{}", $name), |b| {
                b.iter(|| Buffer::<$n, true>::decode(black_box(en_bytes), 'S'));
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

/// A benchmark for prefixed checksum decoding functions.
fn bench_decode_check_prefixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_check_prefixed_const");

    macro_rules! bench {
        ($name:expr, $n:expr, $sample:expr) => {
            let en = c32::encode_check_prefixed(*$sample, 'S', 0).unwrap();
            let en_bytes = en.as_bytes();
            group.bench_function(
                f!("decode_check_prefixed_const_{}", $name),
                |b| {
                    b.iter(|| {
                        Buffer::<$n, true, Check>::decode(
                            black_box(en_bytes),
                            'S',
                        )
                    });
                },
            );
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
    bench_decode,
    bench_decode_check,
    bench_decode_prefixed,
    bench_decode_check_prefixed
);

criterion_main!(benches);
