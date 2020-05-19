# Teapot benches

This directory contains benchmarks that are present in the thesis. They can either be run by invoking the `run_benches.sh` script (with correct parameters), or by running `cargo run` (in debug mode) and `cargo run --release` (in release mode).

Measured results are present in the `benches` directory. For convenience, compiled release binary builds are available in `builds` directory. It might be required to set the `VY_TEAPOT_DEVICE_INDEX` (default: `0`) environment variable to get the examples to run on the correct GPU device.

The binaries need to be either run from the source of the project (e.g. the directory this readme is in) or the `VY_TEAPOT_VERT_SHADER` (default: `src/assets/vert.spv`) and `VY_TEAPOT_FRAG_SHADER` (default: `src/assets/frag.spv`) environment variables need to be set.

Note that to compile for windows, the dependency on termion needs to be disabled in `edwardium_logger`, so comment the `edwardium_logger = "1.1"` line in `Cargo.toml` and uncomment the one underneath that disables default features and select the uncolored stderr target.