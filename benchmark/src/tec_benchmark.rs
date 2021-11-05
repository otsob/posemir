use std::env;
use std::path::Path;

use criterion::{BenchmarkId, Criterion};
use criterion::SamplingMode::Flat;

use point_set_algorithms::point_set::point::Point2dF;
use point_set_algorithms::tec_algorithm::TecAlgorithm;

use crate::data_loader;

pub fn run_tec_benchmarks<T: TecAlgorithm<Point2dF>>(algorithm: &T, algorithm_name: &str, config: &data_loader::Config, c: &mut Criterion) {
    let data_path = env::var("BENCHMARK_DATA_PATH").unwrap();
    let datasets = data_loader::load_datasets(&Path::new(&data_path), &config);

    let benchmark_prefix = format!("{} - {}", algorithm_name, config.path_str);
    let mut group = c.benchmark_group(&benchmark_prefix);
    group.sampling_mode(Flat);

    for point_set in &datasets {
        let size = point_set.len() as u64;
        group.bench_with_input(BenchmarkId::new(&benchmark_prefix, size), &point_set,
                               |b, &input| b.iter(|| algorithm.compute_tecs(input)));
    }

    group.finish();
}
