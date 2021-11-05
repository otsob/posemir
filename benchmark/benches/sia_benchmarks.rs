use criterion::{Criterion, criterion_group, criterion_main};

use benchmark::data_loader;
use benchmark::mtp_benchmark;
use point_set_algorithms::sia::SIA;

fn sia_benchmarks_with_random(c: &mut Criterion) {
    let config = data_loader::Config::default_counts(String::from("random/random_points_"));
    mtp_benchmark::run_mtp_benchmarks(&SIA {}, "SIA", &config, c);
}

fn sia_benchmarks_with_min_pattern_count(c: &mut Criterion) {
    let config = data_loader::Config::default_counts(String::from("min_pattern_count/min_pattern_count_"));
    mtp_benchmark::run_mtp_benchmarks(&SIA {}, "SIA", &config, c);
}

fn sia_benchmarks_with_max_pattern_count(c: &mut Criterion) {
    let config = data_loader::Config::default_counts(String::from("max_pattern_count/max_pattern_count_"));
    mtp_benchmark::run_mtp_benchmarks(&SIA {}, "SIA", &config, c);
}

criterion_group!(name = sia_benchmarks;
    config = Criterion::default().sample_size(20);
    targets = sia_benchmarks_with_random, sia_benchmarks_with_min_pattern_count, sia_benchmarks_with_max_pattern_count);
criterion_main!(sia_benchmarks);
