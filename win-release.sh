#!/bin/sh

# Alternatively:
# export CARGO_FEATURE_PURE=1
# cargo build --target x86_64-pc-windows-msvc --release

cargo build --target x86_64-pc-windows-gnu --release