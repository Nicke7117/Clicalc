use std::io::{self, Write};
mod math;
use math::Math;
mod functions;

fn main() {
    loop {
        let mut input = String::new();
        print!("Enter a math expression >> ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from STDIN");
        let mut math = Math::new(input.trim().to_string());
        math.evaluate();
    }
}
