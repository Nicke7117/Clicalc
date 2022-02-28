pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> f64 {
    a / b
}

pub fn power(a: f64, b: f64) -> f64 {
    a.powi(b as i32)
}

pub fn modulus(a: f64, b: f64) -> f64 {
    a % b
}
