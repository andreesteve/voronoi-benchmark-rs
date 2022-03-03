use super::{RANGE, generate_random_points};
use criterion::{BatchSize, Bencher};
use voronoi::*;

fn prepare(size: usize) -> Vec<Point> {
    generate_random_points(size)
        .map(|(x, y)| Point::new(x, y))
        .collect()
}

fn build(sites: Vec<Point>) -> (Vec<Vec<Point>>, DCEL) {
    let dcell = voronoi::voronoi(sites, 2.0 * RANGE);
    (make_polygons(&dcell), dcell)
}

pub fn create_benchmark_fn(b: &mut Bencher, size: usize) {
    b.iter_batched(
        || prepare(size),
        |s| build(s),
        BatchSize::SmallInput);
}