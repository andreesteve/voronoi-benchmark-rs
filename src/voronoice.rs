use super::{RANGE, generate_random_points};
use criterion::{BatchSize, Bencher};
use voronoice::*;

fn prepare(size: usize) -> Vec<Point> {
    generate_random_points(size)
        .map(|(x, y)| Point { x, y })
        .collect()
}

fn build(sites: Vec<Point>) -> Voronoi {
    VoronoiBuilder::default()
        .set_bounding_box(BoundingBox::new(Point { x: 0.0, y: 0.0 }, 2.0 * RANGE, 2.0 * RANGE))
        .set_sites(sites)
        .build()
        .unwrap()
}

pub fn create_benchmark_fn(b: &mut Bencher, size: usize) {
    b.iter_batched(
        || prepare(size),
        |s| build(s),
        BatchSize::SmallInput);
}