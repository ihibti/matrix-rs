use std::ops::{Add, Mul};
pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: Add<Output = V> +  Mul<f32, Output = V>,
{
    u * (1.0 - t) + v * t
}

#[cfg(test)]
mod tests {
    use super::lerp;
    use crate::Matrix;
    use crate::Vector;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn lerp_scalar_endpoints_and_midpoint() {
        assert_eq!(lerp(0.0_f32, 1.0, 0.0), 0.0);
        assert_eq!(lerp(0.0_f32, 1.0, 1.0), 1.0);
        assert_eq!(lerp(0.0_f32, 1.0, 0.5), 0.5);
        assert!(approx(lerp(21.0_f32, 42.0, 0.3), 27.3));
    }

    #[test]
    fn lerp_vector() {
        let u = Vector::from([2.0_f32, 1.0]);
        let v = Vector::from([4.0_f32, 2.0]);
        let r = lerp(u, v, 0.3);
        assert!(approx(r.as_slice()[0], 2.6));
        assert!(approx(r.as_slice()[1], 1.3));
    }

    #[test]
    fn lerp_matrix() {
        let u: Matrix<f32> = Matrix::from([[2., 1.], [3., 4.]]);
        let v: Matrix<f32> = Matrix::from([[20., 10.], [30., 40.]]);
        let r = lerp(u, v, 0.5);
        assert!(approx(r.get(0, 0), 11.));
        assert!(approx(r.get(0, 1), 5.5));
        assert!(approx(r.get(1, 0), 16.5));
        assert!(approx(r.get(1, 1), 22.));
    }
}
