# Machinecode

Execute machine code in hex form.

This repo includes both a Rust package and a utility called `jitrun`.

This is a port from [Go](https://github.com/xyproto/jit) to Rust.

## Example use

Return the number `42`:

    cargo run -p jitrun -- examples/42.mc

Calculate the square root of `1024`:

    cargo run -p jitrun -- examples/sqrt.mc

## General info

* Version: 0.1.0
* License: BSD-3
