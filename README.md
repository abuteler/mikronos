# Micronos

[![dependency status](https://deps.rs/repo/github/abuteler/micronos/status.svg)](https://deps.rs/repo/github/abuteler/micronos/)
[![Build Status](https://github.com/abuteler/micronos/workflows/CI/badge.svg)](https://github.com/abuteler/micronos/actions?workflow=CI)

Lost traction? Lost grip of time? Reset. Start anywhere. Start somewhere.

A cross-platform time-awareness-and-management Windows-first GUI application written in Rust, and meant first and mostly to be used as an overlay desktop app to help with the aforementioned purpose.

## Setup
This project was jump-started by way of [egui](https://github.com/emilk/egui/)'s [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) template.


## Testing locally
Since I'm working under WSL2, the building of this project requires [cross](https://github.com/cross-rs/cross) or something similar, which is also why the build status is currently broken, and some of github action's default settings are currently commented out. I plan to find a way to fix those at some point, but it's not at the top of my priority list for now.

Beyond following the regular rust installation movements, if you're also under WSL2, you'll need to:
1. `cargo install cross --git https://github.com/cross-rs/cross`
2. `cross build --release --target x86_64-pc-windows-gnu`

## Preview

This is how the app looks and runs at this point in time:

{screen-record capture here}

(TBC)
