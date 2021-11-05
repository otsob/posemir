use criterion::{Criterion, criterion_group, criterion_main};

use benchmark::data_loader;
use benchmark::tec_benchmark;
use point_set_algorithms::siatec::SIATEC;

fn siatec_benchmarks_with_random(c: &mut Criterion) {
    let config = data_loader::Config::default_counts(String::from("random/random_points_"));
    tec_benchmark::run_tec_benchmarks(&SIATEC { remove_duplicates: false }, "SIATEC", &config, c);
}

fn siatec_benchmarks_with_min_pattern_count(c: &mut Criterion) {
    let config = data_loader::Config::default_counts(String::from("min_pattern_count/min_pattern_count_"));
    tec_benchmark::run_tec_benchmarks(&SIATEC { remove_duplicates: false }, "SIATEC", &config, c);
}

fn siatec_benchmarks_with_max_pattern_count(c: &mut Criterion) {
    let config = data_loader::Config::default_counts(String::from("max_pattern_count/max_pattern_count_"));
    tec_benchmark::run_tec_benchmarks(&SIATEC { remove_duplicates: false }, "SIATEC", &config, c);
}

// SIATEC benchmarks without duplicate removal
criterion_group!(name = siatec_benchmarks;
    config = Criterion::default().sample_size(10);
    targets = siatec_benchmarks_with_random, siatec_benchmarks_with_min_pattern_count, siatec_benchmarks_with_max_pattern_count);
criterion_main!(siatec_benchmarks);
