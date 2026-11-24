use crate::scalar::Scalar;
use crate::vector::Vector;
use std::fmt::{self};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<K: Scalar> {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) data: Vec<K>,
}

impl<K: Scalar> Matrix<K> {
    pub fn new(rows: usize, cols: usize, data: Vec<K>) -> Self {
        assert!(rows > 0 && cols > 0, "Matrix dimensions must be > 0");
        assert_eq!(
            data.len(),
            rows * cols,
            "data length must equal rows * cols"
        );
        Self { rows, cols, data }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn get(&self, r: usize, c: usize) -> K {
        debug_assert!(r < self.rows && c < self.cols);
        self.data[c * self.rows + r]
    }

    pub fn set(&mut self, r: usize, c: usize, v: K) {
        debug_assert!(r < self.rows && c < self.cols);
        self.data[c * self.rows + r] = v;
    }

    pub fn identity(n: usize) -> Self {
        let mut data = vec![K::zero(); n * n];
        for i in 0..n {
            data[i * n + i] = K::one();
        }
        Self {
            rows: n,
            cols: n,
            data,
        }
    }

    pub fn ensure_conformable(&self, rhs: &Matrix<K>) {
        assert_eq!(self.shape(), rhs.shape(), "Different shapes");
    }

    pub fn add(&self, rhs: &Matrix<K>) -> Matrix<K> {
        self.ensure_conformable(rhs);
        let a = self
            .data
            .iter()
            .zip(&rhs.data)
            .map(|(v, u)| *v + *u)
            .collect();
        Matrix::new(self.rows, self.cols, a)
    }

    pub fn sub(&self, rhs: &Matrix<K>) -> Matrix<K> {
        self.ensure_conformable(rhs);
        let a = self
            .data
            .iter()
            .zip(&rhs.data)
            .map(|(v, u)| *v - *u)
            .collect();
        Matrix::new(self.rows, self.cols, a)
    }

    pub fn add_assign(&mut self, rhs: &Matrix<K>) {
        self.ensure_conformable(rhs);
        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a += b);
    }

    pub fn sub_assign(&mut self, rhs: &Matrix<K>) {
        self.ensure_conformable(rhs);
        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a -= b);
    }

    pub fn scl_assign(&mut self, s: K) {
        self.data.iter_mut().for_each(|e| *e *= s);
    }

    pub fn scl(&self, s: K) -> Matrix<K> {
        let new = self.data.iter().map(|&v| v * s).collect();
        Matrix::new(self.rows, self.cols, new)
    }

    pub fn mul_vec(&self, v: &Vector<K>) -> Vector<K> {
        assert_eq!(self.cols(), v.size(),);
        let mut acc = vec![K::zero(); self.rows()];
        for (i, a) in acc.iter_mut().enumerate() {
            for j in 0..self.cols() {
                *a = self.get(i, j).mul_add(v.data[j], *a)
            }
        }
        Vector::new(acc)
    }

    pub fn mul_mat(&self, rhs: &Matrix<K>) -> Matrix<K> {
        assert_eq!(self.cols, rhs.rows(), "Mul_mat: dimension mismatch");
        let m = self.rows();
        let n = self.cols();
        let p = rhs.cols();
        let mut ret = vec![K::zero(); m * p];
        for j in 0..p {
            for i in 0..m {
                let mut acc = K::zero();
                for k in 0..n {
                    acc = self.get(i, k).mul_add(rhs.get(k, j), acc);
                }
                ret[j * m + i] = acc;
            }
        }
        Matrix {
            rows: m,
            cols: p,
            data: ret,
        }
    }

    pub fn trace(&self) -> K {
        assert!(self.is_square(), "Trace: Matrix must be square");
        let mut tr = K::zero();
        for i in 0..self.rows() {
            tr += self.get(i, i)
        }
        tr
    }

    pub fn transpose(&self) -> Matrix<K> {
        let (m, n) = (self.rows, self.cols);
        let mut acc = vec![K::zero(); n * m];
        for c in 0..m {
            for r in 0..n {
                acc[c * n + r] = self.get(c, r);
            }
        }
        Matrix {
            rows: n,
            cols: m,
            data: acc,
        }
    }
}

impl<K: Scalar, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(rows: [[K; C]; R]) -> Self {
        assert!(R > 0 && C > 0, "Matrix dimensions must be > 0");
        let mut data = Vec::with_capacity(R * C);
        for c in 0..C {
            for row in &rows {
                data.push(row[c]);
            }
        }
        Self {
            rows: R,
            cols: C,
            data,
        }
    }
}

impl<K: Scalar> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows {
            write!(f, "[")?;
            for c in 0..self.cols {
                if c > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.get(r, c))?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl<K: Scalar> std::ops::Add for &Matrix<K> {
    type Output = Matrix<K>;
    fn add(self, rhs: Self) -> Self::Output {
        <Matrix<K>>::add(self, rhs)
    }
}

impl<K: Scalar> std::ops::Add for Matrix<K> {
    type Output = Matrix<K>;
    fn add(self, rhs: Self) -> Self::Output {
        <Matrix<K>>::add(&self, &rhs)
    }
}

impl<K: Scalar> std::ops::AddAssign<&Matrix<K>> for Matrix<K> {
    fn add_assign(&mut self, rhs: &Self) {
        <Matrix<K>>::add_assign(self, rhs);
    }
}

impl<K: Scalar> std::ops::Sub for &Matrix<K> {
    type Output = Matrix<K>;
    fn sub(self, rhs: Self) -> Self::Output {
        <Matrix<K>>::sub(self, rhs)
    }
}

impl<K: Scalar> std::ops::Sub for Matrix<K> {
    type Output = Matrix<K>;
    fn sub(self, rhs: Self) -> Self::Output {
        <Matrix<K>>::sub(&self, &rhs)
    }
}

impl<K: Scalar> std::ops::SubAssign<&Matrix<K>> for Matrix<K> {
    fn sub_assign(&mut self, rhs: &Self) {
        <Matrix<K>>::sub_assign(self, rhs);
    }
}

impl<K: Scalar> std::ops::Mul<K> for &Matrix<K> {
    type Output = Matrix<K>;
    fn mul(self, rhs: K) -> Self::Output {
        <Matrix<K>>::scl(self, rhs)
    }
}

impl<K: Scalar> std::ops::Mul<K> for Matrix<K> {
    type Output = Matrix<K>;
    fn mul(self, rhs: K) -> Self::Output {
        <Matrix<K>>::scl(&self, rhs)
    }
}

impl<K: Scalar> std::ops::MulAssign<K> for Matrix<K> {
    fn mul_assign(&mut self, rhs: K) {
        <Matrix<K>>::scl_assign(self, rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_2d_array_stores_column_major() {
        let m: Matrix<f32> = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        assert_eq!(m.shape(), (2, 2));
        assert_eq!(m.data, vec![1.0, 3.0, 2.0, 4.0]);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 1), 2.0);
        assert_eq!(m.get(1, 0), 3.0);
        assert_eq!(m.get(1, 1), 4.0);
    }

    #[test]
    fn from_rectangular_literal_stores_column_major() {
        let m: Matrix<f32> = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
        assert_eq!(m.shape(), (2, 3));
        assert_eq!(m.data, vec![1., 4., 2., 5., 3., 6.]);
    }

    #[test]
    fn display_row_by_row() {
        let m: Matrix<f32> = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        assert_eq!(format!("{}", m), "[1, 2]\n[3, 4]\n");
    }

    #[test]
    fn identity_is_correct() {
        let i: Matrix<f32> = Matrix::identity(3);
        assert_eq!(i.shape(), (3, 3));
        for r in 0..3 {
            for c in 0..3 {
                assert_eq!(i.get(r, c), if r == c { 1.0 } else { 0.0 });
            }
        }
    }

    #[test]
    fn is_square_check() {
        let sq: Matrix<f32> = Matrix::from([[1., 0.], [0., 1.]]);
        let nsq: Matrix<f32> = Matrix::from([[1., 0., 0.], [0., 1., 0.]]);
        assert!(sq.is_square());
        assert!(!nsq.is_square());
    }

    #[test]
    fn add_matrices() {
        let mut v = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let d = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let mut u = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let b = &v + &u;
        u.scl_assign(2.0);
        assert_eq!(b, u);
        assert_eq!(u, d.scl(2.0));
        v += &d;
        assert_eq!(v, b);
    }

    #[test]
    fn mul_vec_test() {
        let u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., -4.0]))
    }
    #[test]
    fn mul_vec_asymmetric() {
        // Catches transpose/index-swap bugs (symmetric matrices hide them).
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Vector::from([5., 6.]);
        assert_eq!(u.mul_vec(&v), Vector::from([17., 39.]));
    }

    #[test]
    fn mul_vec_rectangular() {
        // 2x3 * 3-vec -> 2-vec. Catches wrong dim check, output size, loop bounds.
        let u = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
        let v = Vector::from([1., 2., 3.]);
        assert_eq!(u.mul_vec(&v), Vector::from([14., 32.]));
    }

    #[test]
    fn mul_vec_identity() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., 2.]));
    }
    #[test]
    fn mul_mat_identity() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 2.], [3., 4.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[1., 2.], [3., 4.]]));
    }

    #[test]
    fn mul_mat_asymmetric() {
        // Catches transpose/index-swap bugs.
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[5., 6.], [7., 8.]]);
        // [[1*5+2*7, 1*6+2*8], [3*5+4*7, 3*6+4*8]] = [[19, 22], [43, 50]]
        assert_eq!(u.mul_mat(&v), Matrix::from([[19., 22.], [43., 50.]]));
    }

    #[test]
    fn mul_mat_rectangular() {
        // (2x3) * (3x2) -> (2x2). Catches wrong output dims, dim check, loop bounds.
        let u = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
        let v = Matrix::from([[7., 8.], [9., 10.], [11., 12.]]);
        // [[1*7+2*9+3*11, 1*8+2*10+3*12], [4*7+5*9+6*11, 4*8+5*10+6*12]]
        // = [[58, 64], [139, 154]]
        assert_eq!(u.mul_mat(&v), Matrix::from([[58., 64.], [139., 154.]]));
    }

    #[test]
    fn mul_mat_non_square_result() {
        // (3x2) * (2x4) -> (3x4). Catches assuming result is square.
        let u = Matrix::from([[1., 2.], [3., 4.], [5., 6.]]);
        let v = Matrix::from([[1., 0., 1., 0.], [0., 1., 0., 1.]]);
        assert_eq!(
            u.mul_mat(&v),
            Matrix::from([[1., 2., 1., 2.], [3., 4., 3., 4.], [5., 6., 5., 6.],])
        );
    }

    #[test]
    fn trace_identity_2() {
        let m: Matrix<f32> = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(m.trace(), 2.0);
    }

    #[test]
    fn trace_3x3() {
        let m: Matrix<f32> = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(m.trace(), 9.0);
    }

    #[test]
    fn trace_negative() {
        let m: Matrix<f32> = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(m.trace(), -21.0);
    }

    #[test]
    fn transpose_square() {
        let m: Matrix<f32> = Matrix::from([[1., 2.], [3., 4.]]);
        let t = m.transpose();
        assert_eq!(t, Matrix::from([[1., 3.], [2., 4.]]));
    }

    #[test]
    fn transpose_rectangular() {
        let m: Matrix<f32> = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
        let t = m.transpose();
        assert_eq!(t, Matrix::from([[1., 4.], [2., 5.], [3., 6.]]));
        assert_eq!(t.shape(), (3, 2));
    }
}
