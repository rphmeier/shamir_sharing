extern crate num;
extern crate rand;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

use clap::App;
use num::{Zero, One};
use rand::{Rng, OsRng};


mod field;

use field::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    x: Value,
    y: Value,
}

#[derive(Serialize, Deserialize)]
pub struct JsonPoint {
    x: String,
    y: String,
}

impl<'a> From<&'a Point> for JsonPoint {
    fn from(p: &'a Point) -> Self {
        JsonPoint {
            x: p.x.to_hex_string(),
            y: p.y.to_hex_string(),            
        }
    }
}

impl JsonPoint {
    fn to_point(&self) -> Result<Point, ::field::ParseBytesError> {
        Ok(Point {
            x: Value::parse_bytes(self.x.as_bytes(), 16)?,
            y: Value::parse_bytes(self.y.as_bytes(), 16)?,            
        })
    }
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
// t: number of points necessary to recreate secret.
// n: number of points to create.
fn secret_sharing(secret: Value, t: usize, n: usize) -> Vec<Point> {
    assert!(t <= n);
    assert!(t > 1);

    let mut rng = OsRng::new().expect("Failed to acquire secure randomness");

    // make sure we have a degree-n polynomial with all non-zero coefficients.
    let coefficients = make_nonzero_inputs(&mut rng, t - 1);

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
    make_nonzero_inputs(&mut rng, n).into_iter().map(apply_polynomial).collect()
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

fn cli() -> Result<(), String> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        let secret = matches.value_of("secret").expect("required");
        let n = matches.value_of("number").expect("required");
        let t = matches.value_of("threshold").expect("required");


        let secret = Value::parse_bytes(secret.as_bytes(), 16)
            .map_err(|e| format!("{:?}", e))?;
        let n: usize = n.parse().map_err(|e| format!("{}", e))?;
        let t: usize = t.parse().map_err(|e| format!("{}", e))?;

        for point in secret_sharing(secret, t, n) {
            let json_point = JsonPoint::from(&point);
            let json = ::serde_json::to_string(&json_point)
                .expect("serialization will not fail.");

            println!("{}", json);
        }
    }

    if let Some(_) = matches.subcommand_matches("restore") {
        use std::io::{self, Read};

        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)
            .map_err(|e| format!("Error reading from stdin: {}", e))?;

        let points = buffer.lines()
            .map(|line| ::serde_json::from_str::<JsonPoint>(&line))
            .map(|res| res.map_err(|e| format!("Malformatted JSON: {}", e)))
            .map(|res| res.and_then(|jp| jp.to_point().map_err(|e| format!("Malformatted point: {:?}", e))))
            .collect::<Result<Vec<_>, _>>()?;

        let secret = extract_secret(&points);

        println!("{}", secret.to_hex_string());
    }

    Ok(())
}

fn main() {
    use std::io::Write;

    if let Err(e) = cli() {
        let _ = write!(::std::io::stderr(), "{}", e);
    }
}