#!/bin/bash
export RUSTFLAGS="-Z print-type-sizes"
cargo check
