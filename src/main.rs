extern crate num;
extern crate clap;
extern crate rand;

use num::{Zero, One};
use rand::{Rng, OsRng};

mod field;
use field::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    x: Value,
    y: Value,
}

// make a list of non-zero inputs
fn make_nonzero_inputs(rng: &mut OsRng, len: usize) -> Vec<Value> {
    loop {
        let inputs = (0..len).map(|_| rng.gen::<Value>()).collect::<Vec<_>>();
        if !inputs.iter().any(|x| x.is_zero()) {
            break inputs;
        }
    }
}

// Create a set of points for secret sharing.
// n: number of points necessary to recreate secret.
// k: number of points to create.
fn secret_sharing(secret: Value, n: usize, k: usize) -> Vec<Point> {
    assert!(n <= k);
    assert!(n > 1);

    let mut rng = OsRng::new().expect("Failed to acquire secure randomness");

    // make sure we have a degree-n polynomial with all non-zero coefficients.
    let coefficients = make_nonzero_inputs(&mut rng, n - 1);

    let apply_polynomial = |x: Value| {
        let mut out = secret.clone();
        
        let input = x.clone();
        let mut x_to_the = x.clone();

        for coeff in coefficients.iter().cloned() {
            let term = coeff * x_to_the.clone();
            out = out + term;
            x_to_the = x_to_the * x.clone();
        }

        Point { x: input, y: out }
    };

    assert!(apply_polynomial(Value::zero()).y == secret, 
        "secret not embedded in the polynomial correctly");

    // apply the function to k different non-zero inputs
    make_nonzero_inputs(&mut rng, k).into_iter().map(apply_polynomial).collect()
}

// evaluates the j'th langrange basis polynomial at 0.
fn lagrange_basis_at_zero(j: usize, points: &[Point]) -> Value {
    points.iter().enumerate().fold(Value::one(), |accum, (i, point)| {
        if i == j { return accum } // basis polynomials have no term for p[j].

        // (0 - point.x) / (points[j].x - point.x) = m_i
        let numerator = Value::zero() - point.x.clone();
        let denominator = points[j].x.clone() - point.x.clone();
            
        accum * (numerator / denominator)
    }).into()
}

// accepts an input of points, without duplicates in the x-coordinate, and outputs the constant term of the
// n-degree polynomial created by interpolating them.
fn extract_secret(points: &[Point]) -> Value {
    let mut accum = Value::zero();
    for (j, point) in points.iter().enumerate() {
        let term = point.y.clone() * lagrange_basis_at_zero(j, points);
        accum = accum + term;
    }

    accum
}

fn main() {
    let n = 100;
    let k = 150;
    let secret = Value::from(0xabcdefdeadbeefu64);

    let points = secret_sharing(secret.clone(), n, k);

    println!("Secret = {}", secret);

    for point in &points {
        println!(
            "{{ x: {}, y: {} }}", 
            point.x,
            point.y,
        );
    }

    for point_group in points.windows(n) {
        let extracted = extract_secret(point_group);

        println!("Extracted = {}", extracted);
    }
}