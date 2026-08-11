#!/usr/bin/env bash

set -e

RUSTFLAGS="-Awarnings" cargo run --release --example keyboard_io_uring
#RUSTFLAGS="-Awarnings" cargo run --release --example keyboard
#RUSTFLAGS="-Awarnings" cargo run --release --example mouse
RUSTFLAGS="-Awarnings" cargo run --release --example mouse_io_uring

exit 0