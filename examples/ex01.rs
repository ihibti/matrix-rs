use matrix::Vector;

fn main() {
    // Basic example: linear combination of basis vectors
    println!("=== Basic: Linear combination of basis vectors ===");
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    println!("10*e1 - 2*e2 + 0.5*e3:");
    println!(
        "{}",
        Vector::linear_combination(&[e1, e2, e3], &[10., -2., 0.5])
    );
    // Expected:
    // [10.]
    // [-2.]
    // [0.5]

    // Basic example: linear combination of arbitrary vectors
    println!("\n=== Basic: Linear combination of arbitrary vectors ===");
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("10*v1 - 2*v2:");
    println!(
        "{}",
        Vector::linear_combination(&[v1.clone(), v2.clone()], &[10., -2.])
    );
    // Expected:
    // [10.]
    // [0.]
    // [230.]

    // Demonstration: Zero coefficients produce zero vector
    println!("\n=== Property: Zero coefficients ===");
    println!("0*v1 + 0*v2:");
    let v1_again = Vector::from([1., 2., 3.]);
    let v2_again = Vector::from([0., 10., -100.]);
    println!(
        "{}",
        Vector::linear_combination(&[v1_again, v2_again], &[0., 0.])
    );

    // Demonstration: Selecting a single vector
    println!("\n=== Property: Selecting one vector ===");
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([4., 5., 6.]);
    let v3 = Vector::from([7., 8., 9.]);
    println!("0*v1 + 1*v2 + 0*v3 (should equal v2):");
    println!(
        "{}",
        Vector::linear_combination(&[v1, v2, v3], &[0., 1., 0.])
    );

    // Demonstration: Negative coefficients
    println!("\n=== Example: Negative coefficients ===");
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([1., 1., 1.]);
    println!("-3*v1 + 2*v2:");
    println!("{}", Vector::linear_combination(&[v1, v2], &[-3., 2.]));

    // Demonstration: Fractional coefficients
    println!("\n=== Example: Fractional coefficients ===");
    let v1 = Vector::from([2., 4., 6.]);
    let v2 = Vector::from([1., 1., 1.]);
    println!("0.5*v1 + 1.5*v2:");
    println!("{}", Vector::linear_combination(&[v1, v2], &[0.5, 1.5]));

    // Demonstration: Additivity property
    println!("\n=== Property: Additivity ===");
    println!("Computing: 2*e1 + 3*e2 + 5*e3 in one go vs. separately:");
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);

    let combined = Vector::linear_combination(&[e1.clone(), e2.clone(), e3.clone()], &[2., 3., 5.]);
    println!("Combined:");
    println!("{}", combined);

    let part1 = Vector::linear_combination(&[e1], &[2.]);
    let part2 = Vector::linear_combination(&[e2], &[3.]);
    let part3 = Vector::linear_combination(&[e3], &[5.]);
    let separate = part1.add(&part2).add(&part3);
    println!("Separate (sum of parts):");
    println!("{}", separate);
}
