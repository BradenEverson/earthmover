#!/bin/sh
cargo build -p earthmover-simulation --example physics --target x86_64-pc-windows-gnu --release &&
cp target/x86_64-pc-windows-gnu/release/examples/physics.exe . &&
exec ./physics.exe "$@"
