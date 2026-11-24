use matrix::Matrix;
use matrix::Vector;

fn main() {
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [2.]
    let u = Matrix::from([[2., 0.], [0., 2.]]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [8.]
    // [4.]
    let u = Matrix::from([[2., -2.], [-2., 2.]]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [-4.]
}
