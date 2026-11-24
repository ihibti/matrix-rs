pub mod decomp;
pub mod error;
pub mod lerp;
pub mod matrix;
pub mod scalar;
pub mod vector;

pub use error::MatrixError;
pub use matrix::Matrix;
pub use scalar::{RealScalar, Scalar};
pub use vector::Vector;

pub fn approx(u: f32, v: f32) {
    assert!((u - v).abs() < 0.00001)
}
