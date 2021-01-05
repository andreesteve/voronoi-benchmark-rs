use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use voronoi_benchmark::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Build Voronoi Cells");
    let iterations: [usize;8] = [100, 500, 1_000, 10_000, 100_000, 500_000, 1_000_000, 2_000_000];

    // limitting to 1000 due to execution time
    for size in iterations.iter().take(3) {
        group.bench_with_input(
            BenchmarkId::new("delauney2d", size),
            size,
            |b, &bench_size| delaunay2d::create_benchmark_fn(b, bench_size)
        );
    }

    for size in iterations.iter() {
        group.bench_with_input(
            BenchmarkId::new("voronoi", size),
            size,
            |b, &bench_size| voronoi::create_benchmark_fn(b, bench_size)
        );

        group.bench_with_input(
            BenchmarkId::new("voronoice", size),
            size,
            |b, &bench_size| voronoice::create_benchmark_fn(b, bench_size)
        );

        group.bench_with_input(
            BenchmarkId::new("voronator", size),
            size,
            |b, &bench_size| voronator::create_benchmark_fn(b, bench_size)
        );
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);