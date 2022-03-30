pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

pub fn power(a: f64, b: f64) -> f64 {
    a.powi(b as i32)
}

pub fn modulus(a: f64, b: f64) -> f64 {
    a % b
}

pub fn sine(a: f64) -> f64 {
    a.sin()
}

pub fn cosine(a: f64) -> f64 {
    a.cos()
}

pub fn tangent(a: f64) -> f64 {
    a.tan()
}
