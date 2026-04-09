use broken_app::{algo, average_positive, normalize, sum_even};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn bench_sum_even(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_even");
    for size in [1_000, 10_000, 50_000] {
        let data: Vec<i64> = (0..size).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| sum_even(data))
        });
    }
    group.finish();
}

fn bench_fib(c: &mut Criterion) {
    let mut group = c.benchmark_group("fib");
    for n in [20, 25, 32] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| algo::slow_fib(n))
        });
    }
    group.finish();
}

fn bench_dedup(c: &mut Criterion) {
    let mut group = c.benchmark_group("dedup");
    for size in [500, 1_000, 5_000] {
        let data: Vec<u64> = (0..size).flat_map(|n| [n, n]).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter_batched(
                || data.clone(),
                |v| algo::slow_dedup(&v),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn bench_normalize(c: &mut Criterion) {
    let mut group = c.benchmark_group("normalize");
    for size in [1_000, 50_000, 500_000] {
        let text: String = (0..size)
            .map(|i| if i % 7 == 0 { ' ' } else { 'a' })
            .collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &text, |b, text| {
            b.iter(|| normalize(text))
        });
    }
    group.finish();
}

fn bench_average_positive(c: &mut Criterion) {
    let mut group = c.benchmark_group("average_positive");
    for size in [1_000, 10_000, 50_000] {
        let data: Vec<i64> = (-((size / 2) as i64)..((size / 2) as i64)).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| average_positive(data))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_sum_even,
    bench_fib,
    bench_dedup,
    bench_normalize,
    bench_average_positive
);
criterion_main!(benches);
