#!/bin/sh

# for macos
export DYLD_FALLBACK_LIBRARY_PATH="$VULKAN_SDK/lib:${DYLD_FALLBACK_LIBRARY_PATH:-}"

PLATFORM=${1:-"PLATFORM"}
DEVICE_INDEX=${2:-0}

rm Cargo.toml
ln -s ST-Cargo.toml Cargo.toml
VY_TEAPOT_DEVICE_INDEX=$DEVICE_INDEX cargo run --bin ash --release > "benches/${PLATFORM}/raw_ash.txt"
VY_TEAPOT_DEVICE_INDEX=$DEVICE_INDEX cargo run --bin vulkayes --release > "benches/${PLATFORM}/raw_vulkayes_ST.txt"

rm Cargo.toml
ln -s MT-Cargo.toml Cargo.toml
VY_TEAPOT_DEVICE_INDEX=$DEVICE_INDEX cargo run --bin vulkayes --release > "benches/${PLATFORM}/raw_vulkayes_MT.txt"