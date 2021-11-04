use criterion::{Criterion, criterion_group, criterion_main};

use benchmark::config;
use benchmark::data_loader;
use benchmark::mtp_benchmark;
use point_set_algorithms::siar::SIAR;

fn siar_benchmarks_with_random(c: &mut Criterion) {
    let config = data_loader::Config {
        min: config::MIN,
        max: config::MAX,
        step: config::STEP,
        path_str: String::from("random/random_points_"),
    };

    let algorithm = SIAR { r: 3 };
    mtp_benchmark::run_mtp_benchmarks(&algorithm, "SIAR(3)", &config, c);
}

fn siar_benchmarks_with_min_pattern_count(c: &mut Criterion) {
    let config = data_loader::Config {
        min: config::MIN,
        max: config::MAX,
        step: config::STEP,
        path_str: String::from("min_pattern_count/min_pattern_count_"),
    };


    let algorithm = SIAR { r: 3 };
    mtp_benchmark::run_mtp_benchmarks(&algorithm, "SIAR(3)", &config, c);
}

fn siar_benchmarks_with_max_pattern_count(c: &mut Criterion) {
    let config = data_loader::Config {
        min: config::MIN,
        max: config::MAX,
        step: config::STEP,
        path_str: String::from("max_pattern_count/max_pattern_count_"),
    };

    let algorithm = SIAR { r: 3 };
    mtp_benchmark::run_mtp_benchmarks(&algorithm, "SIAR(3)", &config, c);
}

criterion_group!(name = siar_benchmarks;
    config = Criterion::default().sample_size(20);
    targets = siar_benchmarks_with_random, siar_benchmarks_with_min_pattern_count, siar_benchmarks_with_max_pattern_count);
criterion_main!(siar_benchmarks);
