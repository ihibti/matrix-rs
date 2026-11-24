use matrix::{Vector,Matrix};
fn main() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add_assign(&v);
    println!("{}", u);
    // [7]
    // [10]
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub_assign(&v);
    println!("{}", u);
    // [-3]
    // [-4]
    let mut u = Vector::from([2., 3.]);
    u.scl_assign(2.);
    println!("{}", u);
    // [4]
    // [6]
    let  u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    println!("{}", u.add(&v));
    // [8.0, 6.0]
    // [1.0, 6.0]
    let  u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    println!("{}", u.sub(&v));
    // [-6.0, -2.0]
    // [5.0, 2.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl_assign(2.0);
    println!("{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]
}
