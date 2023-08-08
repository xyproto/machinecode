# Machinecode

Execute machine code in hex form.

This repo includes both a Rust package and a utility called `jitrun`.

It was inspired by this post: [Hello, JIT World: The Joy of Simple JITs](http://blog.reverberate.org/2012/12/hello-jit-world-joy-of-simple-jits.html).

This is a port from [Go](https://github.com/xyproto/jit) to Rust.

## Example use

Return the number `42`:

    cargo run -p jitrun -- examples/42.mc

Calculate the square root of `1024`:

    cargo run -p jitrun -- examples/sqrt.mc

## General info

* Version: 0.1.0
* License: BSD-3
