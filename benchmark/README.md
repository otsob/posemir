# Benchmarks for Point Set MIR algorithms

This crate implements benchmarks and benchmarks utilities for point set discovery algorithms. The data sets are included
in the `benches/data` directory and can be downloaded using [DVC](https://dvc.org) with the
`gsuite` add-on. Alternatively you can download the benchmark data
from [this link](https://drive.google.com/file/d/1btpk5ft9fVrpEFz8y5f4yJyRsokeGpzU/view?usp=drive_link)
as a zip file.

Running the benchmarks requires setting the environment variable `BENCHMARK_DATA_PATH` to the absolute path of the
directory `benchmark/benches/data` inside this repository.

The benchmarks are implemented using [criterion](https://github.com/bheisler/criterion.rs).
To run the benchmarks, execute `cargo bench`
inside this directory (`benchmark`). For running a specific benchmark, use: `cargo bench --bench=<benchmark_name>`.

The benchmarks data set sizes can be set with the following environment variables:

- `BENCHMARK_DATASET_MIN_SIZE`: minimum size of datasets to use
- `BENCHMARK_DATASET_MAX_SIZE`: maximum size of datasest to use
- `BENCHMARK_DATASET_STEP_SIZE`: increment of how many datasets to use between min and max (must be a multiple of 100)

