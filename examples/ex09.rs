use matrix::Matrix;

fn main() {
    let u = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    println!("{}", u.transpose());
    let v = Matrix::from([[1., 2.], [3., 4.], [5., 6.]]);
    println!("{}", v.transpose());
}
