use crate::scalar::{RealScalar, Scalar};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<K: Scalar> {
    pub(crate) data: Vec<K>,
}

impl<K: Scalar> Vector<K> {
    pub fn new(data: Vec<K>) -> Self {
        assert!(!data.is_empty(), "Vector must not be empty");
        Self { data }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn as_slice(&self) -> &[K] {
        &self.data
    }

    pub fn add_assign(&mut self, u: &Vector<K>) {
        self.ensure_same_len(u);
        self.data
            .iter_mut()
            .zip(&u.data)
            .for_each(|(a, &b)| *a += b);
    }

    pub fn sub_assign(&mut self, u: &Vector<K>) {
        self.ensure_same_len(u);
        self.data
            .iter_mut()
            .zip(&u.data)
            .for_each(|(a, &b)| *a -= b);
    }

    pub fn scl_assign(&mut self, s: K) {
        self.data.iter_mut().for_each(|x| *x *= s);
    }

    pub fn scl(&self, s: K) -> Vector<K> {
        self.data.iter().map(|&x| x * s).collect()
    }

    pub fn add(&self, u: &Vector<K>) -> Vector<K> {
        self.ensure_same_len(u);
        self.data
            .iter()
            .zip(&u.data)
            .map(|(&a, &b)| a + b)
            .collect()
    }

    pub fn sub(&self, u: &Vector<K>) -> Vector<K> {
        self.ensure_same_len(u);
        self.data
            .iter()
            .zip(&u.data)
            .map(|(&a, &b)| a - b)
            .collect()
    }

    pub fn ensure_same_len(&self, rhs: &Self) {
        assert_eq!(self.data.len(), rhs.data.len(), "vector dimension mismatch");
    }

    pub fn linear_combination(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
        assert!(!u.is_empty(), "at least one vector required");
        let len_vec = u[0].size();
        let amount_vec = u.len();
        assert_eq!(
            amount_vec,
            coefs.len(),
            "number of coefficients ({}) must match number of vectors ({})",
            coefs.len(),
            amount_vec
        );

        assert!(
            u.iter().all(|v| v.size() == len_vec),
            "vector dimension mismatch"
        );
        let mut acc = vec![K::zero(); len_vec];
        for (v, &coef) in u.iter().zip(coefs.iter()) {
            for (acc_cell, &value) in acc.iter_mut().zip(&v.data) {
                *acc_cell = Scalar::mul_add(value, coef, *acc_cell);
            }
        }
        Vector { data: acc }
    }

    pub fn dot(&self, v: &Vector<K>) -> K {
        self.ensure_same_len(v);
        let mut ret: K;
        ret = <K>::zero();
        self.data
            .iter()
            .zip(&v.data)
            .for_each(|(a, b)| ret += *(a) * *(b));
        ret
    }

    pub fn norm_1(&self) -> K::Real {
        self.data
            .iter()
            .fold(K::Real::zero(), |ret, x| ret + x.abs())
    }

    pub fn norm(&self) -> K::Real {
        self.data
            .iter()
            .fold(K::Real::zero(), |ret, x| x.abs().mul_add(x.abs(), ret))
            .sqrt()
    }

    pub fn norm_inf(&self) -> K::Real {
        let ret = K::Real::zero();
        self.data.iter().fold(ret, |ret, x| ret.max(x.abs()))
    }

    pub fn angle_cos(u: &Vector<K>, v: &Vector<K>) -> K::Real {
        u.ensure_same_len(v);
        let x = u.norm();
        let y = v.norm();
        if x == K::Real::zero() || y == K::Real::zero() {
            panic!("angle_cos: Zero Vector has no angle")
        }
        u.dot(v).re() / (y * x)
    }

    pub fn cross_product(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
        Vector::ensure_same_len(v, u);
        assert_eq!(
            v.size(),
            3,
            "Cross Product: Vectors can only be of dimension 3"
        );
        let a = u.data[1].mul_add(v.data[2], -(u.data[2] * v.data[1]));
        let b = u.data[2].mul_add(v.data[0], -(u.data[0] * v.data[2]));
        let c = u.data[0].mul_add(v.data[1], -(u.data[1] * v.data[0]));
        Vector::from([a, b, c])
    }
}

impl<K: Scalar, const N: usize> From<[K; N]> for Vector<K> {
    fn from(arr: [K; N]) -> Self {
        assert!(N > 0, "Vector must not be empty");
        Self { data: arr.to_vec() }
    }
}

impl<K: Scalar> std::ops::Mul<K> for &Vector<K> {
    type Output = Vector<K>;
    fn mul(self, rhs: K) -> Self::Output {
        self.scl(rhs)
    }
}

impl<K: Scalar> std::ops::Mul<K> for Vector<K> {
    type Output = Vector<K>;
    fn mul(self, rhs: K) -> Self::Output {
        self.scl(rhs)
    }
}

impl<K: Scalar> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in &self.data {
            writeln!(f, "[{}]", x)?;
        }
        Ok(())
    }
}

impl<K: Scalar> std::ops::Add for &Vector<K> {
    type Output = Vector<K>;
    fn add(self, rhs: Self) -> Self::Output {
        <Vector<K>>::add(self, rhs)
    }
}

impl<K: Scalar> std::ops::Add for Vector<K> {
    type Output = Vector<K>;
    fn add(self, rhs: Self) -> Self::Output {
        <Vector<K>>::add(&self, &rhs)
    }
}

impl<K: Scalar> std::ops::Sub for &Vector<K> {
    type Output = Vector<K>;
    fn sub(self, rhs: Self) -> Self::Output {
        <Vector<K>>::sub(self, rhs)
    }
}

impl<K: Scalar> std::ops::Sub for Vector<K> {
    type Output = Vector<K>;
    fn sub(self, rhs: Self) -> Self::Output {
        <Vector<K>>::sub(&self, &rhs)
    }
}

impl<K: Scalar> std::ops::AddAssign<&Vector<K>> for Vector<K> {
    fn add_assign(&mut self, rhs: &Vector<K>) {
        <Vector<K>>::add_assign(self, rhs);
    }
}

impl<K: Scalar> std::ops::SubAssign<&Vector<K>> for Vector<K> {
    fn sub_assign(&mut self, rhs: &Vector<K>) {
        <Vector<K>>::sub_assign(self, rhs);
    }
}

impl<K: Scalar> std::ops::MulAssign<K> for Vector<K> {
    fn mul_assign(&mut self, rhs: K) {
        <Vector<K>>::scl_assign(self, rhs);
    }
}

impl<K: Scalar> FromIterator<K> for Vector<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        Vector::new(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(u: f32, v: f32) {
        assert!((u - v).abs() < 0.00001)
    }

    #[test]
    fn from_array() {
        let v: Vector<f32> = Vector::from([1.0, 2.0, 3.0]);
        assert_eq!(v.size(), 3);
        assert_eq!(v.as_slice(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn display_one_per_line() {
        let v: Vector<f32> = Vector::from([7.0, 10.0]);
        let s = format!("{}", v);
        assert_eq!(s, "[7]\n[10]\n");
    }

    #[test]
    #[should_panic(expected = "Vector must not be empty")]
    fn empty_panics() {
        let _v: Vector<f32> = Vector::new(vec![]);
    }

    #[test]
    fn add_vectors() {
        let mut u = Vector::from([2.0, 3.0]);
        let v = Vector::from([5.0, 7.0]);
        u.add_assign(&v);
        assert_eq!(u.as_slice(), &[7.0, 10.0]);
    }

    #[test]
    fn sub_vectors() {
        let mut u = Vector::from([2.0, 3.0]);
        let v = Vector::from([5.0, 7.0]);
        u.sub_assign(&v);
        assert_eq!(u.as_slice(), &[-3.0, -4.0]);
    }

    #[test]
    fn scale_vector() {
        let mut u = Vector::from([2.0, 3.0]);
        u.scl_assign(2.0);
        assert_eq!(u.as_slice(), &[4.0, 6.0]);
    }

    #[test]
    fn pure_add_returns_new() {
        let u = Vector::from([2.0, 3.0]);
        let v = Vector::from([5.0, 7.0]);
        let s = u.add(&v);
        assert_eq!(s.as_slice(), &[7.0, 10.0]);
        assert_eq!(u.as_slice(), &[2.0, 3.0], "u should be unchanged");
    }

    #[test]
    fn ops_traits() {
        let u = Vector::from([2.0, 3.0]);
        let v = Vector::from([5.0, 7.0]);
        let s = &u + &v;
        assert_eq!(s.as_slice(), &[7.0, 10.0]);
        let d = &u - &v;
        assert_eq!(d.as_slice(), &[-3.0, -4.0]);
        let p = &u * 2.0;
        assert_eq!(p.as_slice(), &[4.0, 6.0]);
        let mut k = Vector::from([2.0, 3.0]);
        k *= 2.0;
        assert_eq!(k.as_slice(), &[4.0, 6.0]);
        k -= &u;
        assert_eq!(k.as_slice(), &[2.0, 3.0]);
        k += &u;
        assert_eq!(k.as_slice(), &[4.0, 6.0]);
    }

    #[test]
    fn linear_combination_arbitrary_vectors() {
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        let result = Vector::linear_combination(&[v1, v2], &[10., -2.]);
        assert_eq!(result, Vector::from([10., 0., 230.]));
    }

    #[test]
    fn linear_combination_zero_coefficients() {
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([4., 5., 6.]);
        let result = Vector::linear_combination(&[v1, v2], &[0., 0.]);
        assert_eq!(result, Vector::from([0., 0., 0.]));
    }

    #[test]
    fn linear_combination_single_vector() {
        let v = Vector::from([2., 3., 5.]);
        let result = Vector::linear_combination(&[v], &[4.]);
        assert_eq!(result, Vector::from([8., 12., 20.]));
    }

    #[test]
    fn linear_combination_mixed_signs() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);

        let result = Vector::linear_combination(&[e1, e2, e3], &[5., -3., 2.]);
        assert_eq!(result, Vector::from([5., -3., 2.]));
    }

    #[test]
    #[should_panic(expected = "at least one vector required")]
    fn linear_combination_empty_vectors() {
        let _result = Vector::<f32>::linear_combination(&[], &[]);
    }

    #[test]
    #[should_panic(expected = "number of coefficients")]
    fn linear_combination_mismatched_counts() {
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([4., 5., 6.]);
        let _result = Vector::linear_combination(&[v1, v2], &[1.]);
    }

    #[test]
    #[should_panic(expected = "vector dimension mismatch")]
    fn linear_combination_different_sized_vectors() {
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([4., 5.]);
        let _result = Vector::linear_combination(&[v1, v2], &[1., 1.]);
    }
    #[test]
    fn dot_product_test() {
        let v1 = Vector::from([3.0, 4.0]);
        let v2 = Vector::from([5.0, 0.0]);
        assert_eq!(Vector::dot(&v1, &v2), 15.0);
    }

    #[test]
    fn test_norms() {
        let v1 = Vector::from([3.0, -4.0, 2.0]);
        assert_eq!(9.0, v1.norm_1());
        assert_eq!(5.3851647, v1.norm());
        assert_eq!(4.0, v1.norm_inf());
    }

    #[test]
    fn test_cos() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);
        approx(1.0, Vector::angle_cos(&u, &v));
        let u = Vector::from([0., 1.]);
        let v = Vector::from([1., 0.]);
        approx(0.0, Vector::angle_cos(&u, &v));
        let u = Vector::from([0., 1.]);
        let v = Vector::from([-1., 0.]);
        approx(0.0, Vector::angle_cos(&u, &v));
        let u = Vector::from([2., 0.]);
        let v = Vector::from([2., 0.]);
        approx(1.0, Vector::angle_cos(&u, &v));
        let u = Vector::from([3., 8., -9.]);
        let v = Vector::from([-9., 3., 8.]);
        approx(Vector::angle_cos(&u, &v), -0.487013);
    }

    #[test]
    fn test_cross_product() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);
        let exp = Vector::from([0., 1., 0.]);
        assert_eq!(Vector::cross_product(&u, &v), exp);
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        let exp = Vector::from([-3., 6., -3.]);
        assert_eq!(Vector::cross_product(&u, &v), exp);
        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        let exp = Vector::from([17., -58., -16.]);
        assert_eq!(Vector::cross_product(&u, &v), exp);
    }
    #[test]
    #[should_panic(expected = "vector dimension mismatch")]
    fn cross_different_sized_vectors() {
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([4., 5.]);
        let _result = Vector::cross_product(&v1, &v2);
    }

    #[test]
    #[should_panic(expected = "Cross Product: Vectors can only be of dimension 3")]
    fn cross_dimension_not_3() {
        let v1 = Vector::from([1., 2.]);
        let v2 = Vector::from([4., 5.]);
        let _result = Vector::cross_product(&v1, &v2);
    }
}
