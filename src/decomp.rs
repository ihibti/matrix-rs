use crate::MatrixError;
use crate::matrix::Matrix;
use crate::scalar::Scalar;

impl<K: Scalar> Matrix<K> {
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut m = self.clone();
        let (rows, cols) = (m.rows, m.cols);
        let mut r = 0;
        let mut lead = 0;
        while r < rows && lead < cols {
            let mut i = r;
            while i < rows && m.get(i, lead).abs() == K::Real::zero() {
                i += 1;
            }
            if i == rows {
                lead += 1;
                continue;
            }
            swap_rows(&mut m, i, r);
            let pivot = m.get(r, lead);
            for c in 0..cols {
                let v = m.get(r, c);
                m.set(r, c, v / pivot);
            }
            for j in 0..rows {
                if j == r {
                    continue;
                }
                let factor = m.get(j, lead);
                if factor.abs() == K::Real::zero() {
                    continue;
                }
                for c in 0..cols {
                    let new = m.get(j, c) - factor * m.get(r, c);
                    m.set(j, c, new);
                }
            }
            r += 1;
            lead += 1;
        }
        m
    }

    pub fn determinant(&self) -> K {
        assert!(self.is_square(), "determinant: Matrice must be square");
        if self.cols == 1 {
            return self.get(0, 0);
        }
        let mut m = self.clone();
        let n = self.cols();
        let mut sign_flip = false;
        let mut det = K::one();
        for i in 0..n {
            let mut biggest = K::zero();
            let mut target_row = i;
            for j in i..n {
                if m.get(j, i).abs() > biggest.abs() {
                    target_row = j;
                    biggest = m.get(j, i);
                }
            }
            if biggest.abs() == K::Real::zero() {
                return K::zero();
            }
            if target_row != i {
                sign_flip = !sign_flip;
                swap_rows(&mut m, i, target_row);
            }
            let factor = m.get(i, i);
            det *= factor;
            for row in i + 1..n {
                let quot = m.get(row, i) / factor;
                for col in 0..n {
                    // let sub = m.get(row, col) - quot * m.get(i, col);
                    // same but mul_add
                    let sub = (-quot).mul_add(m.get(i, col), m.get(row, col));
                    m.set(row, col, sub);
                }
            }
        }
        if sign_flip { -det } else { det }
    }

    pub fn inverse(&self) -> Result<Matrix<K>, MatrixError> {
        let n = self.rows;
        let mut m = self.clone();
        let mut inv = Matrix::identity(n);
        for col in 0..n {
            let mut pivot_row = col;
            let mut best = m.get(col, col).abs();
            for r in (col + 1)..n {
                let v = m.get(r, col).abs();
                if v > best {
                    best = v;
                    pivot_row = r;
                }
            }
            if best == K::Real::zero() {
                return Err(MatrixError::Singular);
            }
            swap_rows(&mut m, pivot_row, col);
            swap_rows(&mut inv, pivot_row, col);
            let p = m.get(col, col);
            for c in 0..n {
                let v = m.get(col, c);
                let vinv = inv.get(col, c);
                m.set(col, c, v / p);
                inv.set(col, c, vinv / p);
            }
            for r in 0..n {
                if r == col {
                    continue;
                }
                let factor = m.get(r, col);
                if factor.abs() == K::Real::zero() {
                    continue;
                }
                for c in 0..n {
                    let new = m.get(r, c) - factor * m.get(col, c);
                    let new_inv = inv.get(r, c) - factor * inv.get(col, c);
                    m.set(r, c, new);
                    inv.set(r, c, new_inv);
                }
            }
        }
        Ok(inv)
    }

    pub fn rank(&self) -> usize {
        let r = self.row_echelon();
        let delt = K::Real::from_f32(1e-6);
        let mut count = 0;
        for row in 0..r.rows {
            let mut nonzero = false;
            for col in 0..r.cols {
                if r.get(row, col).abs() > delt {
                    nonzero = true;
                    break;
                }
            }
            if nonzero {
                count += 1;
            }
        }
        count
    }
}

fn swap_rows<K: Scalar>(m: &mut Matrix<K>, i: usize, j: usize) {
    if i == j {
        return;
    }
    let cols = m.cols;
    for c in 0..cols {
        let a = m.get(i, c);
        let b = m.get(j, c);
        m.set(i, c, b);
        m.set(j, c, a);
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix;

    fn approx_eq(a: &Matrix<f32>, b: &Matrix<f32>, eps: f32) -> bool {
        if a.shape() != b.shape() {
            return false;
        }
        a.data.iter().zip(&b.data).all(|(x, y)| (x - y).abs() < eps)
    }

    #[test]
    fn rref_identity() {
        let m: Matrix<f32> = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert!(approx_eq(&m.row_echelon(), &m, 1e-5));
    }

    #[test]
    fn rref_2x2_full_rank() {
        let m: Matrix<f32> = Matrix::from([[1., 2.], [3., 4.]]);
        let exp: Matrix<f32> = Matrix::from([[1., 0.], [0., 1.]]);
        assert!(approx_eq(&m.row_echelon(), &exp, 1e-5));
    }

    #[test]
    fn rref_rank_deficient() {
        let m: Matrix<f32> = Matrix::from([[1., 2.], [2., 4.]]);
        let exp: Matrix<f32> = Matrix::from([[1., 2.], [0., 0.]]);
        assert!(approx_eq(&m.row_echelon(), &exp, 1e-5));
    }

    #[test]
    fn rref_subject_3x5() {
        let m: Matrix<f32> = Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        let exp: Matrix<f32> = Matrix::from([
            [1., 0.625, 0., 0., -12.166_667],
            [0., 0., 1., 0., -3.6666667],
            [0., 0., 0., 1., 29.5],
        ]);
        assert!(approx_eq(&m.row_echelon(), &exp, 1e-3));
    }

    #[test]
    fn det_2x2_singular() {
        let m: Matrix<f32> = Matrix::from([[1., -1.], [-1., 1.]]);
        assert!((m.determinant()).abs() < 1e-5);
    }

    #[test]
    fn det_3x3_diag() {
        let m: Matrix<f32> = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        assert!((m.determinant() - 8.0).abs() < 1e-5);
    }

    #[test]
    fn det_3x3_known() {
        let m: Matrix<f32> = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        assert!((m.determinant() + 174.0).abs() < 1e-3);
    }

    #[test]
    fn det_4x4_known() {
        let m: Matrix<f32> = Matrix::from([
            [8., 5., -2., 4.],
            [4., 2.5, 20., 4.],
            [8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ]);
        assert!((m.determinant() - 1032.0).abs() < 1e-1);
    }
    #[test]
    fn inverse_identity() {
        let m: Matrix<f32> = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        let inv = m.inverse().unwrap();
        let exp: Matrix<f32> = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        let approx = |a: &Matrix<f32>, b: &Matrix<f32>| {
            a.data
                .iter()
                .zip(&b.data)
                .all(|(x, y)| (x - y).abs() < 1e-5)
        };
        assert!(approx(&inv, &exp));
    }

    #[test]
    fn inverse_diag2() {
        let m: Matrix<f32> = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        let inv = m.inverse().unwrap();
        let exp: Matrix<f32> = Matrix::from([[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]]);
        let approx = |a: &Matrix<f32>, b: &Matrix<f32>| {
            a.data
                .iter()
                .zip(&b.data)
                .all(|(x, y)| (x - y).abs() < 1e-5)
        };
        assert!(approx(&inv, &exp));
    }

    #[test]
    fn inverse_3x3_known() {
        let m: Matrix<f32> = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        let inv = m.inverse().unwrap();
        let exp: Matrix<f32> = Matrix::from([
            [0.649_425_27, 0.097_701_15, -0.655_172_4],
            [-0.781_609_2, -0.126_436_78, 0.965_517_2],
            [0.143_678_16, 0.074_712_64, -0.206_896_56],
        ]);
        let approx = |a: &Matrix<f32>, b: &Matrix<f32>| {
            a.data
                .iter()
                .zip(&b.data)
                .all(|(x, y)| (x - y).abs() < 1e-3)
        };
        assert!(approx(&inv, &exp));
    }

    #[test]
    fn inverse_singular_errs() {
        let m: Matrix<f32> = Matrix::from([[1., -1.], [-1., 1.]]);
        assert!(matches!(m.inverse(), Err(crate::MatrixError::Singular)));
    }

    #[test]
    fn rank_identity_3() {
        let m: Matrix<f32> = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(m.rank(), 3);
    }

    #[test]
    fn rank_subject_3x4() {
        let m: Matrix<f32> = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
        assert_eq!(m.rank(), 2);
    }

    #[test]
    fn rank_subject_4x3() {
        let m: Matrix<f32> =
            Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
        assert_eq!(m.rank(), 3);
    }
}
