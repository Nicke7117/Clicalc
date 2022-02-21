#[derive(Debug)]
pub struct Math {
    expression: String,
    stack: Vec<String>,
    queue: Vec<String>,
    reverse_polish_notation: Vec<String>,
}

impl Math {
    pub fn new(expression: String) -> Math {
        Math {
            expression,
            stack: Vec::new(),
            queue: Vec::new(),
            reverse_polish_notation: Vec::new(),
        }
    }
    pub fn evaluate(&mut self) -> Option<f64> {
        self.convert_to_reverse_polish_notation();
        self.evaluate_reverse_polish_notation();
    }
}
