use criterion::{Criterion, criterion_group, criterion_main};

use benchmark::{config, tec_benchmark};
use benchmark::data_loader;
use point_set_algorithms::siatec::SIATEC;

fn siatec_benchmarks_with_random(c: &mut Criterion) {
    let config = data_loader::Config {
        min: config::MIN,
        max: config::MAX,
        step: config::STEP,
        path_str: String::from("random/random_points_"),
    };

    tec_benchmark::run_tec_benchmarks(&SIATEC {}, "SIATEC", &config, c);
}

fn siatec_benchmarks_with_min_pattern_count(c: &mut Criterion) {
    let config = data_loader::Config {
        min: config::MIN,
        max: config::MAX,
        step: config::STEP,
        path_str: String::from("min_pattern_count/min_pattern_count_"),
    };

    tec_benchmark::run_tec_benchmarks(&SIATEC {}, "SIATEC", &config, c);
}

fn siatec_benchmarks_with_max_pattern_count(c: &mut Criterion) {
    let config = data_loader::Config {
        min: config::MIN,
        max: config::MAX,
        step: config::STEP,
        path_str: String::from("max_pattern_count/max_pattern_count_"),
    };

    tec_benchmark::run_tec_benchmarks(&SIATEC {}, "SIATEC", &config, c);
}

criterion_group!(name = siatec_benchmarks;
    config = Criterion::default().sample_size(10);
    targets = siatec_benchmarks_with_random, siatec_benchmarks_with_min_pattern_count, siatec_benchmarks_with_max_pattern_count);
criterion_main!(siatec_benchmarks);
