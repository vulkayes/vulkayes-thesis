#!/bin/sh

set -e

# for macos
export DYLD_FALLBACK_LIBRARY_PATH="$VULKAN_SDK/lib:${DYLD_FALLBACK_LIBRARY_PATH:-}"

PLATFORM=${1:-"PLATFORM"}
DEVICE_INDEX=${2:-0}

python3 benches/script/no_sleep.py &
nosleep_pid=$!

echo ">> Running raw_ash bench"
VY_TEAPOT_DEVICE_INDEX=$DEVICE_INDEX cargo run --bin ash --release > "benches/${PLATFORM}/raw_ash.txt"
echo ">> Running raw_vulkayes_ST bench"
VY_TEAPOT_DEVICE_INDEX=$DEVICE_INDEX cargo run --bin vulkayes --release > "benches/${PLATFORM}/raw_vulkayes_ST.txt"

sed -e 's/# "multi_thread"/"multi_thread"/' -i.bak Cargo.toml
echo ">> Running raw_vulkayes_MT bench"
VY_TEAPOT_DEVICE_INDEX=$DEVICE_INDEX cargo run --bin vulkayes --release > "benches/${PLATFORM}/raw_vulkayes_MT.txt"
mv Cargo.toml.bak Cargo.toml

# uniform 1000
sed -e 's/pub const NUMBER_OF_UNIFORM_DATA: usize = 1;/pub const NUMBER_OF_UNIFORM_DATA: usize = 1000;/' -i.bak src/data.rs

echo ">> Running raw_ash_uniform1000 bench"
VY_TEAPOT_DEVICE_INDEX=$DEVICE_INDEX cargo run --bin ash --release > "benches/${PLATFORM}/raw_ash_uniform1000.txt"
echo ">> Running raw_vulkayes_ST_uniform1000 bench"
VY_TEAPOT_DEVICE_INDEX=$DEVICE_INDEX cargo run --bin vulkayes --release > "benches/${PLATFORM}/raw_vulkayes_ST_uniform1000.txt"

sed -e 's/# "multi_thread"/"multi_thread"/' -i.bak Cargo.toml
echo ">> Running raw_vulkayes_MT_uniform1000 bench"
VY_TEAPOT_DEVICE_INDEX=$DEVICE_INDEX cargo run --bin vulkayes --release > "benches/${PLATFORM}/raw_vulkayes_MT_uniform1000.txt"
mv Cargo.toml.bak Cargo.toml

mv src/data.rs.bak src/data.rs

kill $nosleep_pid