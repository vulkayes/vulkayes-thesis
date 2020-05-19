# Teapot benches

This directory contains benchmarks that are present in the thesis. They can either be run by invoking the `run_benches.sh` script (with correct parameters), or by running `cargo run` (in debug mode) and `cargo run --release` (in release mode).

Measured results are present in the `benches` directory. For convenience, compiled binary builds are available in `builds` directory. It might be required to set the `VY_TEAPOT_DEVICE_INDEX=N` (default: `N = 0`) environment variable to get the examples to run on the correct GPU device.