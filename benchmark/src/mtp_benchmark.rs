/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use std::env;
use std::path::Path;

use criterion::SamplingMode::Flat;
use criterion::{BenchmarkId, Criterion};

use posemir_discovery::algorithm::MtpAlgorithm;
use posemir_discovery::point_set::mtp::Mtp;
use posemir_discovery::point_set::point::Point2Df64;

use crate::data_loader;

pub fn run_mtp_benchmarks<T: MtpAlgorithm<Point2Df64>>(
    algorithm: &T,
    algorithm_name: &str,
    config: &data_loader::Config,
    c: &mut Criterion,
) {
    let data_path = env::var("BENCHMARK_DATA_PATH").unwrap();
    let datasets = data_loader::load_datasets(&Path::new(&data_path), &config);

    let group_name = format!("{} - {}", algorithm_name, config.path_str);
    let mut group = c.benchmark_group(&group_name);
    group.sampling_mode(Flat);

    let on_output = |mtp: Mtp<Point2Df64>| {
        criterion::black_box(mtp);
    };

    for point_set in &datasets {
        let size = point_set.len() as u64;
        group.bench_with_input(BenchmarkId::new("", size), &point_set, |b, &input| {
            b.iter(|| {
                algorithm.compute_mtps_to_output(input, on_output);
            })
        });
    }

    group.finish();
}
