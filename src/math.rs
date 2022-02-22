#[derive(Debug)]
pub struct Math {
    expression: String,
    tokens: Vec<String>,
    reverse_polish_notation: Vec<String>,
}

fn is_operator(token: &str) -> bool {
    match token {
        "+" => true,
        "-" => true,
        "*" => true,
        "/" => true,
        "^" => true,
        "(" => true,
        ")" => true,
        _ => panic!("Invalid operator: {}", token),
    }
}

impl Math {
    pub fn new(expression: String) -> Math {
        Math {
            expression,
            tokens: Vec::new(),
            reverse_polish_notation: Vec::new(),
        }
    }

    pub fn evaluate(&mut self) {
        self.split_into_tokens();
    }

    fn split_into_tokens(&mut self) {
        let mut token = String::new();
        for (i, c) in self.expression.chars().enumerate() {
            if c.is_numeric() {
                if i == self.expression.len() - 1 {
                    token.push(c);
                    self.tokens.push(token.clone());
                } else {
                    token.push(c);
                }
            } else if c.is_whitespace() {
                if !token.is_empty() {
                    self.tokens.push(token.clone());
                    token.clear();
                }
            } else if is_operator(c.to_string().as_str()) {
                if !token.is_empty() {
                    self.tokens.push(token.clone());
                    token.clear();
                }
                self.tokens.push(c.to_string());
            }
        }
    }
}
