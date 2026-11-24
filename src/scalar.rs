use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Scalar:
    Copy
    + Debug
    + Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
{
    type Real: RealScalar;

    fn zero() -> Self;
    fn one() -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn abs(self) -> Self::Real;
    fn conjugate(self) -> Self;
    fn re(self) -> Self::Real;
    fn from_f32(x: f32) -> Self;
}

pub trait RealScalar: Scalar<Real = Self> + PartialOrd {
    fn sqrt(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn max(self, other: Self) -> Self;
}

impl Scalar for f32 {
    type Real = f32;

    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn mul_add(self, a: Self, b: Self) -> Self {
        f32::mul_add(self, a, b)
    }
    fn abs(self) -> Self::Real {
        f32::abs(self)
    }
    fn conjugate(self) -> Self {
        self
    }
    fn re(self) -> Self::Real {
        self
    }
    fn from_f32(x: f32) -> Self {
        x
    }
}

impl RealScalar for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
    fn powi(self, n: i32) -> Self {
        f32::powi(self, n)
    }
    fn max(self, other: Self) -> Self {
        f32::max(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_scalar_basics() {
        assert_eq!(f32::zero(), 0.0);
        assert_eq!(f32::one(), 1.0);
        assert_eq!(2.0_f32.mul_add(3.0, 1.0), 7.0);
        assert_eq!((-3.0_f32).abs(), 3.0);
        assert_eq!(2.5_f32.conjugate(), 2.5);
        assert_eq!(2.5_f32.re(), 2.5);
        assert_eq!(f32::from_f32(1.5), 1.5);
    }

    #[test]
    fn f32_real_basics() {
        assert_eq!(4.0_f32.sqrt(), 2.0);
        assert_eq!(2.0_f32.powi(3), 8.0);
        assert_eq!(1.0_f32.max(2.0), 2.0);
    }
}
