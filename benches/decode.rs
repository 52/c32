use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;

macro_rules! gen_decode_benchmark {
    ($($size:expr),*) => {
        fn benchmark_decode(c: &mut Criterion) {
            let mut group = c.benchmark_group("decode");
            $(
                let input = std::fs::read(concat!("samples/c32_", $size, ".in")).unwrap();
                let encoded = black_box(c32::encode(&input));

                group.bench_function(
                    BenchmarkId::new("c32", $size),
                    |b| b.iter(|| c32::decode(&encoded).unwrap())
                );
            )*
            group.finish();
        }
    };
}

gen_decode_benchmark!(
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
    "s_4m"
);

criterion_group!(benches, benchmark_decode);
criterion_main!(benches);
