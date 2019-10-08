# <img src="https://raw.githubusercontent.com/NeoBirth/micromath/develop/img/micromath.png" width="640">

[![Crate][crate-img]][crate-link]
[![Docs][docs-img]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0/MIT Licensed][license-image]][license-link]
[![Gitter Chat][gitter-image]][gitter-link]

Embedded Rust (i.e. `#![no_std]`-friendly) math library featuring fast, safe
floating point approximations for common arithmetic operations, 2D and 3D
vector types, statistical analysis, and quaternions.

Optimizes for performance and small code size at the cost of precision.

[Documentation][docs-link]

## Requirements

- Rust 1.31+

## Features

- [`f32` extension]:
  - Fast approximations:
    - [ ] `asin`
    - [ ] `acos`
    - [x] [atan]
    - [x] [atan2]
    - [x] [cos]
    - [ ] `hypot`
    - [x] [inv]
    - [x] [invsqrt]
    - [x] [ln]
    - [x] [log]
    - [x] [log2]
    - [x] [log10]
    - [x] [powf]
    - [x] [exp]
    - [x] [sin]
    - [x] [sqrt]
    - [x] [tan]
  - `std` polyfills:
    - [x] [abs]
    - [x] [ceil]
    - [x] [floor]
    - [x] [round]
    - [x] [trunc]
    - [x] [fract]
    - [x] [copysign]

- [Algebraic vector types]:
  - 2D:
    - [x] [I8x2]
    - [x] [I16x2]
    - [x] [I32x2]
    - [x] [U8x2]
    - [x] [U16x2]
    - [x] [U32x2]
    - [x] [F32x2]
  - 3D:
    - [x] [I8x3]
    - [x] [I16x3]
    - [x] [I32x3]
    - [x] [U8x3]
    - [x] [U16x3]
    - [x] [U32x3]
    - [x] [F32x3]
- [Statistical analysis]:
  - [x] [mean]
  - [x] [stddev]
  - [x] [trim]
  - [x] [variance]
- [Quaternions]

## Comparisons with other Rust crates

### `libm` crate

The [libm crate] contains a port of MUSL's libm to Rust, providing
high-precision implementations of common arithmetic functions.

`micromath` trades precision for performance, instead using the best-available
approximations of the same functions, implemented using safe conversions
between `f32` and `u32`.

The approximations are generally calculated using a combination of bit
twiddling and magic constants, as opposed to the FPU-heavy approaches used by
`libm`. These approaches are culled from recent academic research papers as
well as older approaches which have been commonly used in games and other
performance critical use cases where approximations are adequate.

### `vek` crate

The [vek crate] provides a rich library for 2D and 3D vector types.
Unfortunately, due to a [number of issues](https://github.com/yoanlcq/vek/issues/20)
including a transitive `std` dependency through the [approx crate],
`vek` does not support `no_std`. According to the crate's author, the potential
fixes are nontrivial (and involve addressing problems such as transcendantal
functions causing overflow panics).

`micromath` has been written from the ground up to support `no_std` use cases.
In fact, it doesn't even have a `std` feature!

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

For more information, please see [CODE_OF_CONDUCT.md].

## License

Copyright © 2019 NeoBirth Developers

Dual licensed under your choice of either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

Incorporates portions of some tests from the [libm crate].
Copyright © 2018 Jorge Aparicio and also dual licensed under the
Apache 2.0 and MIT licenses. 

[crate-img]: https://img.shields.io/crates/v/micromath.svg
[crate-link]: https://crates.io/crates/micromath
[docs-img]: https://docs.rs/micromath/badge.svg
[docs-link]: https://docs.rs/micromath/
[build-image]: https://travis-ci.com/NeoBirth/micromath.svg?branch=develop
[build-link]: https://travis-ci.com/NeoBirth/micromath
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[license-link]: https://github.com/NeoBirth/micromath/blob/develop/LICENSE-APACHE
[gitter-image]: https://badges.gitter.im/NeoBirth/micromath.svg
[gitter-link]: https://gitter.im/NeoBirth/community
[`f32` extension]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html
[atan]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.atan
[atan2]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.atan2
[cos]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.cos
[inv]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.inv
[invsqrt]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.invsqrt
[ln]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.ln
[log]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.log
[log2]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.log2
[log10]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.log10
[powf]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.powf
[exp]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.exp
[sin]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.sin
[sqrt]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.sqrt
[tan]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.tan
[abs]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.abs
[ceil]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.ceil
[floor]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.floor
[round]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.round
[trunc]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.trunc
[fract]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.fract
[copysign]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.copysign
[Algebraic vector types]: https://docs.rs/micromath/latest/micromath/vector/index.html
[I8x2]: https://docs.rs/micromath/latest/micromath/vector/struct.I8x2.html
[I16x2]: https://docs.rs/micromath/latest/micromath/vector/struct.I16x2.html
[I32x2]: https://docs.rs/micromath/latest/micromath/vector/struct.I32x2.html
[U8x2]: https://docs.rs/micromath/latest/micromath/vector/struct.U8x2.html
[U16x2]: https://docs.rs/micromath/latest/micromath/vector/struct.U16x2.html
[U32x2]: https://docs.rs/micromath/latest/micromath/vector/struct.U32x2.html
[F32x2]: https://docs.rs/micromath/latest/micromath/vector/struct.F32x2.html
[I8x3]: https://docs.rs/micromath/latest/micromath/vector/struct.I8x3.html
[I16x3]: https://docs.rs/micromath/latest/micromath/vector/struct.I16x3.html
[I32x3]: https://docs.rs/micromath/latest/micromath/vector/struct.I32x3.html
[U8x3]: https://docs.rs/micromath/latest/micromath/vector/struct.U8x3.html
[U16x3]: https://docs.rs/micromath/latest/micromath/vector/struct.U16x3.html
[U32x3]: https://docs.rs/micromath/latest/micromath/vector/struct.U32x3.html
[F32x3]: https://docs.rs/micromath/latest/micromath/vector/struct.F32x3.html
[Statistical analysis]: https://docs.rs/micromath/latest/micromath/statistics/index.html
[mean]: https://docs.rs/micromath/latest/micromath/statistics/trait.Mean.html
[stddev]: https://docs.rs/micromath/latest/micromath/statistics/trait.StdDev.html
[trim]: https://docs.rs/micromath/latest/micromath/statistics/trim/trait.Trim.html
[variance]: https://docs.rs/micromath/latest/micromath/statistics/trait.Variance.html
[Quaternions]: https://docs.rs/micromath/latest/micromath/quaternion/struct.Quaternion.html
[libm crate]: https://github.com/rust-lang-nursery/libm
[vek crate]: https://github.com/yoanlcq/vek
[approx crate]: https://crates.io/crates/approx
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/NeoBirth/micromath/blob/develop/CODE_OF_CONDUCT.md
