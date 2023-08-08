# Machinecode

Execute machine code in hex form.

This repo includes both a Rust package and a utility called `jitrun`.

It was inspired by this post: [Hello, JIT World: The Joy of Simple JITs](http://blog.reverberate.org/2012/12/hello-jit-world-joy-of-simple-jits.html).

This is a port of [jit](https://github.com/xyproto/jit) from Go to Rust.

## Example use on x86_64

Return the number `42`:

    cargo run -p jitrun -- examples/42.mc

Contents of `42.mc`:

```
// This program moves 42 into eax and returns

b8 2a 00 00 00  // mov 0x2a000000 into the eax register. b8 is the "mov eax" part. 2a is 42.
c3              // return to the caller (the return value is held in eax)
```

Calculate the square root of `1024`:

    cargo run -p jitrun -- examples/sqrt.mc

Contents of `sqrt.mc`:

```
// This program takes the square root of 1024 and returns the answer (in eax), which is 32

b8 00 04 00 00  // mov 1024 (0x0400) into eax (0x00 comes first and then 0x04, because it is little-endian)
f3 0f 2a c0     // mov eax into the xmm0 register
f3 0f 51 c0     // take the square root of the xmm0 register and place it into xmm0
f3 0f 2c c0     // move xmm0 back into eax
c3              // return to the caller (the return value is held in eax)

```

## General info

* Version: 0.1.0
* License: BSD-3
