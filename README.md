<h1 style="text-align: center;">
    <div align="center">earclip-rs</div>
</h1>

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/TimTheBig/earclip-rs/rust.yml?logo=github)
[![Latest version](https://img.shields.io/crates/v/earclip-rs.svg)](https://crates.io/crates/earclip-rs)
[![Documentation](https://docs.rs/earclip-rs/badge.svg)](https://docs.rs/earclip-rs)
[![codecov code coverage](https://codecov.io/gh/TimTheBig/earclip-rs/graph/badge.svg?token=A5RU2AEORU)](https://codecov.io/gh/TimTheBig/earclip-rs)

## About

A fast and small 3D/2D polygon triangulation library with builtin tesselation.

A simplified fork of [OpenS2/earclip](https://github.com/OpenS2/earclip).

## Install

```sh
cargo add earclip-2
```

## The Algorithm

The library implements a modified ear slicing algorithm,
optimized by [z-order curve](http://en.wikipedia.org/wiki/Z-order_curve) hashing
and extended to handle holes, twisted polygons, degeneracies and self-intersections
in a way that doesn't _guarantee_ correctness of triangulation,
but attempts to always produce acceptable results for practical data.

It's based on ideas from
[FIST: Fast Industrial-Strength Triangulation of Polygons](http://www.cosy.sbg.ac.at/~held/projects/triang/triang.html) by Martin Held
and [Triangulation by Ear Clipping](http://www.geometrictools.com/Documentation/TriangulationByEarClipping.pdf) by David Eberly.

## Usage

```rs
use earclip_2::earclip_float;

let polygon: &[Vec<&[f64]>] = &[vec![&[0.0, 0.0, 0.0], &[1.0, 0.0, 0.0], &[0.0, 1.0, 0.0]]];
let (vertices, indices) = earclip_float(polygon, None, None);

assert_eq!(vertices, vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
assert_eq!(indices, vec![1, 2, 0]);
```

> [!NOTE]  
> Safety Unsafe code is forbidden by a #![forbid(unsafe_code)] attribute in the root of the rust library.

---

## Development

### Requirements

You need the tool `tarpaulin` to generate the coverage report. Install it using the following command:

```sh
cargo install cargo-tarpaulin
```

### Running Tests

To run the tests, use the following command:

```sh
## basic test
cargo test
# live testing
bacon test
```

### Generating Coverage Report

To generate the coverage report, use the following command:

```sh
cargo tarpaulin
# faster
cargo tarpaulin --color always --skip-clean
```

## Benchmarks

### Rust

Run the Rust benchmarks using the following command:

```sh
cargo +nightly bench
```
