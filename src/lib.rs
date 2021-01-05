pub mod voronoi;
pub mod delaunay2d;
pub mod voronator;
pub mod voronoice;

const RANGE: f64 = 1.0;

fn generate_random_points(size: usize) -> impl Iterator<Item = (f64, f64)> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let range = rand::distributions::Uniform::new(-RANGE, RANGE);
    (0..size)
        .map(move |_| (rng.sample(range), rng.sample(range)))
}