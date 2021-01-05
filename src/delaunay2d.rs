use super::{RANGE, generate_random_points};
use criterion::{BatchSize, Bencher};
use delaunay2d::*;

fn prepare(size: usize) -> Vec<(f64, f64)> {
    generate_random_points(size)
        .collect()
}

fn build(sites: Vec<(f64, f64)>) -> (Vec<(f64, f64)>, Vec<Vec<usize>>) {
    let mut d = Delaunay2D::new((0.0, 0.0), RANGE);

    for p in sites {
        d.add_point(p);
    }

    d.export_voronoi_regions()
}

pub fn create_benchmark_fn(b: &mut Bencher, size: usize) {
    b.iter_batched(
        || prepare(size),
        |s| build(s),
        BatchSize::SmallInput);
}