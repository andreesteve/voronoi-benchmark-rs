# A naive benchmark of Voronoi implementations in Rust

This is a simplistic benchmark of the different [Voronoi diagram](https://en.wikipedia.org/wiki/Voronoi_diagram) generating crates in [Rust](https://www.rust-lang.org/) I could find on [crates.io](https://crates.io/search?q=voronoi).

## Disclaimer

Please take the results here with a grain of salt. Different crates take different design approaches that are optimized for distinct scenarios and thus will vary in their execution time. For instance, some implementations provide a iterative approach in which the diagram are updated as each site/point is added; others collect all the sites before computing the diagram.

Also not all crates provide the same level of functionality, which may impact their execution time. Please refer to each crate's documentation for more details.

## Motivation

Voronoi diagrams are fun. I wrote my own Voronoi library in Rust, [voronoice](https://github.com/andreesteve/voronoice). Besides that, there are a few crates that provide the ability to generate them. I wanted to understand their differences and performance characteristics, so I created this benchmark to see how they compared.

## Methodology

1. [Criterion.rs](https://github.com/bheisler/criterion.rs) is used to drive the benchmark execution
2. Random sites/points are generated in a preparation step (not measured as part of final result)
3. For each crate a ```create_benchmark_fn``` was implemented to take the generated points and pass it to the library to compute the voronoi diagram based on their documented examples
4. The execution returns all data generated by the library execution to Criterion. This is to prevent the benchmark from measuring any deallocation overhead, as documented [here](https://bheisler.github.io/criterion.rs/book/user_guide/timing_loops.html).
5. Each execution generates its own set of random points (not included in the measure). So crates run with different points, but they are constraint to the same space (-1.0..1.0) and all libraries were configured to use a bounding box that encompases that range

### Fairness

I tried to be as fair as I could when writing the benchmark for each implementation. Having that said, the benchmark function for each library is heavely based on their documentation examples. If you see some code that could be introducing bias to any of the brenchmarks, please do let me know by filing an issue or send a PR!

## Running the benchmark

You can run the benchmark by cloning this repository and executing the following on the terminal:

```> cargo bench```

## Results

Last updated: 2021-01-09

### Crate versions used

| Crate                                             | Version |
| ------------------------------------------------- | ------- |
| [delaunay2d](https://crates.io/crates/delaunay2d) | 0.0.2   |
| [voronator](https://crates.io/crates/voronator)   | 0.1.0   |
| [voronoi](https://crates.io/crates/voronoi)       | 0.1.4   |
| [voronoice](https://crates.io/crates/voronoice)   | 0.0.1   |

### Scenario: Build all Voronoi Cells

This scenario gives a set of N sites (f64,f64) and calculates and returns the polygons representing each Voronoi cell.
Each set of N sites is run 100 times.
The values of N can be found [here](/benches/compare.rs).
This benchmark should run for about 1 hour and take about 1GB of memory at peak.

#### Results Summary

Mean values shown below.

| Crate                                             | N=100  | N=500    | N=1k     | N=10k   | N=100k | N=500k | N=1M | N=2M |
| ------------------------------------------------- | -------|----------|----------|---------|--------|--------|--------|--------|
| [delaunay2d](https://crates.io/crates/delaunay2d) | 690 us | 10214 us | 37864 us | N/A     | N/A    | N/A    | N/A    | N/A    |
| [voronator](https://crates.io/crates/voronator)   | 86 us  | 452 us   | 914 us   | 10.6 ms | 181 ms | 1.16 s | 2.56 s | 5.36 s |
| [voronoi](https://crates.io/crates/voronoi)       | 240 us | 12877 us | 26384 us | 30.2 ms | 416 ms | 2.42 s | 5.10 s | 11.1 s |
| [voronoice](https://crates.io/crates/voronoice)   | 75 us  | 377 us   | 756 us   | 8.1 ms  | 135 ms | 0.84 s | 1.88 s | 4.04 s |


* N/A = not computed (see note below)
* k = 1,000
* M = 1,000,000

**NOTE:** delaunay2d's algorithmic complexity seems to be [non-quasilinear](https://en.wikipedia.org/wiki/Time_complexity#Quasilinear_time), so I resticted its tests to only N in [100, 500, 1000] to keep the entire benchmark within 60 minutes execution time. Due to calculating just a few N, it doesn't show nicely in the graph below. But you can find its report [here](target/criterion/Build%20Voronoi%20Cells/delauney2d/report/index.html).

![](target/criterion/Build%20Voronoi%20Cells/report/lines.svg)

#### Report links

* [Full report link](target/criterion/Build%20Voronoi%20Cells/report/index.html)
