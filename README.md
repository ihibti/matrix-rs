# matrix-rs

[![CI](https://github.com/ihibti/matrix-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/ihibti/matrix-rs/actions/workflows/ci.yml)

Linear algebra from scratch in Rust, with no dependencies: vectors, matrices, Gaussian elimination, determinants, inverses and rank.

I wrote it for 42's *Enter the Matrix: An introduction to Linear Algebra*, a subject by Tristan Duquesne and Luc Lenôtre. The subject itself is not included here.

## Why

Raycasting, machine learning and a lot of numerical code reduce to a few operations on vectors and matrices. Writing each one by hand, within the time and space bounds the subject sets, is the surest way I know to understand what a library call actually does. Fundamentals compound.

## Exercises

| # | Exercise | Where | Time | Space |
|---|---|---|---|---|
| 00 | Add, subtract, scale | [`Vector::{add, sub, scl}`](src/vector.rs), [`Matrix::{add, sub, scl}`](src/matrix.rs) and the operators | O(n) | O(n) |
| 01 | Linear combination | [`Vector::linear_combination`](src/vector.rs) | O(n) | O(n) |
| 02 | Linear interpolation | [`lerp`](src/lerp.rs) | O(n) | O(n) |
| 03 | Dot product | [`Vector::dot`](src/vector.rs) | O(n) | O(n) |
| 04 | Norms (1, 2, ∞) | [`Vector::{norm_1, norm, norm_inf}`](src/vector.rs) | O(n) | O(n) |
| 05 | Cosine | [`Vector::angle_cos`](src/vector.rs) | O(n) | O(n) |
| 06 | Cross product | [`Vector::cross_product`](src/vector.rs) | – | – |
| 07 | Matrix × vector, matrix × matrix | [`Matrix::{mul_vec, mul_mat}`](src/matrix.rs) | O(nm), O(nmp) | O(nm), O(nm + mp + np) |
| 08 | Trace | [`Matrix::trace`](src/matrix.rs) | O(n) | – |
| 09 | Transpose | [`Matrix::transpose`](src/matrix.rs) | O(nm) | O(nm) |
| 10 | Reduced row echelon form | [`Matrix::row_echelon`](src/decomp.rs) | O(n³) | O(n²) |
| 11 | Determinant | [`Matrix::determinant`](src/decomp.rs) | O(n³) | O(n²) |
| 12 | Inverse | [`Matrix::inverse`](src/decomp.rs) | O(n³) | O(n²) |
| 13 | Rank | [`Matrix::rank`](src/decomp.rs) | O(n³) | – |
| 14 | Projection matrix (bonus) | not done yet | | |
| 15 | Complex vector spaces (bonus) | not done yet | | |

Time and space are the bounds the subject sets, and each implementation stays within them. A dash means the subject sets none.

## Design choices

- **Generic scalars.** Everything is generic over `K: Scalar`. `Scalar` carries an associated `type Real: RealScalar`, and `abs`, `re` and `conjugate` go through it, so complex numbers (exercise 15) can be added without touching the algorithms. Only `f32` implements it today.
- **Column-major storage.** `Matrix` stores its entries column by column (`data[c * rows + r]`), while `Matrix::from` takes row-major literals, the way a matrix is written on paper.
- **Fused multiply-add.** `mul_add` is part of `Scalar`. Linear combination, the Euclidean norm, the cross product, both matrix products and the determinant's elimination use it, so each multiply-add rounds once instead of twice.
- **Panics for shapes, `Result` for data.** A shape mismatch is a programming error, so it panics with a message, the way an out-of-range index does. A singular matrix is a property of the data, so `inverse` returns `Result<Matrix<K>, MatrixError>`.
- **No dependencies.**

## Run

```sh
cargo test
cargo run --example ex05
```

Each example runs the subject's test cases for one exercise. The comment under each line is the value the subject expects: f32 output can differ in the last digit, and elimination can print `-0`.

## Status

Exercises 00 to 13 are done. The two bonus exercises, 14 (projection matrix) and 15 (complex vector spaces), are not done yet.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
