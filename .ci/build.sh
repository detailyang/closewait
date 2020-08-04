#! /bin/bash

cd /app
scl enable llvm-toolset-7.0 bash
cargo build --release
