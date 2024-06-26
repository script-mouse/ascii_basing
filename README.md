# ascii_basing
A Rust Library that converts unsigned 32-bit integers ([`u32`](https://doc.rust-lang.org/std/primitive.u32.html)s) to unique [`String`] values that are always smaller or as small as base 10 representations of the given integer and can be decoded back to the exact value before encoding.
`ascii_basing` does this using standard library functions to implement a [Base62](https://en.wikipedia.org/wiki/Base62) encoding and decoding algorithm.
For more information on how to use this crate, check the documentation.