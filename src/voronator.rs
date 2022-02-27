use super::{RANGE, generate_random_points};
use criterion::{BatchSize, Bencher};
use voronator::{delaunator::Point, VoronoiDiagram};

fn prepare(size: usize) -> Vec<(f64, f64)> {
    generate_random_points(size)
        .collect()
}

fn build(sites: Vec<(f64, f64)>) -> VoronoiDiagram<Point> {
    VoronoiDiagram::from_tuple(&(-RANGE, -RANGE), &(-RANGE, -RANGE), &sites).unwrap()
}

pub fn create_benchmark_fn(b: &mut Bencher, size: usize) {
    b.iter_batched(
        || prepare(size),
        |s| build(s),
        BatchSize::SmallInput);
}