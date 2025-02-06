use std::format as f;
use std::fs::read;

use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

const BENCHMARK_SAMPLES: [&str; 16] = [
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

fn benchmark_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");

    for sample in BENCHMARK_SAMPLES {
        let benchmark_id = f!("c32_{sample}");
        let input = read(f!("samples/{benchmark_id}.in")).unwrap();
        let bytes = black_box(&input);

        group.bench_function(benchmark_id, |b| b.iter(|| c32::encode(&bytes)));
    }
}

criterion_group!(benches, benchmark_encode);
criterion_main!(benches);
