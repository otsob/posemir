# Benchmarks for Point Set Repeated Pattern algorithms

This crate implements benchmarks and benchmarks utilities for point set discovery algorithms. The data sets are included
in the `benches/data` directory and can be downloaded using [DVC](https://dvc.org) with the
`gsuite`add-on.

Running the benchmarks requires setting the environment variable `BENCHMARK_DATA_PATH` to the absolute path of the
directory `benchmark/benches/data` inside this repository.

The benchmarks are implemented using [criterion](https://github.com/bheisler/criterion.rs)
and [cargo-criterion](https://github.com/bheisler/cargo-criterion). To run the benchmarks, execute `cargo criterion`
inside this directory (`benchmark`).

The benchmarks data set sizes can be set with the following envinronment variables:

- `BENCHMARK_DATASET_MIN_SIZE`: minimum size of datasets to use
- `BENCHMARK_DATASET_MAX_SIZE`: maximum size of datasest to use
- `BENCHMARK_DATASET_STEP_SIZE`: increment of how many datasets to use between min and max (must be a multiple of 100)

