use crate::functions::basic;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Math {
    expression: String,
    tokens: Vec<String>,
    reverse_polish_notation: Vec<String>,
}

fn is_operator(token: &str) -> bool {
    match token {
        "+" | "-" | "*" | "/" | "^" | "%" | "(" | ")" => true,
        _ => false,
    }
}

fn is_function(token: &str) -> bool {
    match token {
        "sin" | "cos" | "tan" | "log" | "ln" | "sqrt" | "abs" | "floor" | "ceil" | "round" => true,
        _ => false,
    }
}

fn is_constant(token: &str) -> bool {
    match token {
        "pi" | "e" => true,
        _ => false,
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
        if let Err(e) = self.split_into_tokens() {
            println!("{}", e);
            return;
        }
        self.convert_to_reverse_polish_notation();
        if let Err(e) = self.solve_reverse_polish_notation() {
            println!("{}", e);
            return;
        }
    }

    fn split_into_tokens(&mut self) -> Result<(), String> {
        let mut current_token = String::new();
        for token in self.expression.chars() {
            if token.is_whitespace() {
                continue;
            } else if is_operator(token.to_string().as_str()) {
                if current_token.len() > 0 {
                    self.tokens.push(current_token);
                    current_token = String::new();
                }
                self.tokens.push(token.to_string());
            } else {
                current_token.push(token);
            }
        }
        if current_token.len() > 0 {
            self.tokens.push(current_token);
        }
        Ok(())
    }
    fn convert_to_reverse_polish_notation(&mut self) {
        let mut queue: Vec<String> = Vec::new();
        let mut stack: Vec<String> = Vec::new();
        let precedence = HashMap::from([
            ("+", 1),
            ("-", 1),
            ("*", 2),
            ("/", 2),
            ("^", 3),
            ("%", 3),
            ("sin", 1),
            ("cos", 1),
            ("tan", 1),
            ("log", 1),
            ("ln", 1),
            ("sqrt", 1),
            ("abs", 1),
            ("floor", 1),
            ("ceil", 1),
            ("round", 1),
        ]);
        for token in self.tokens.iter() {
            if token.parse::<f64>().is_ok() {
                queue.push(token.to_string())
            } else if is_constant(token.as_str()) {
                match token.as_str() {
                    "pi" => queue.push(std::f64::consts::PI.to_string()),
                    "e" => queue.push(std::f64::consts::E.to_string()),
                    _ => (),
                }
            } else if token == "(" {
                stack.push(token.to_string())
            } else if token == ")" {
                while let Some(popped) = stack.pop() {
                    if popped == "(" {
                        break;
                    } else {
                        queue.push(popped)
                    }
                }
            } else if is_function(token) {
                stack.push(token.to_string())
            } else {
                while !stack.is_empty()
                    && stack.last().unwrap() != "("
                    && precedence.get(&*String::from(token)).unwrap()
                        <= precedence
                            .get(&*String::from(stack.last().unwrap()))
                            .unwrap()
                {
                    queue.push(stack.pop().unwrap());
                }
                stack.push(token.to_string());
            }
        }
        while !stack.is_empty() {
            queue.push(stack.pop().unwrap());
        }
        self.reverse_polish_notation = queue.clone();
    }
    fn solve_reverse_polish_notation(&mut self) -> Result<(), String> {
        let mut stack: Vec<f64> = Vec::new();
        for token in self.reverse_polish_notation.iter() {
            if token.parse::<f64>().is_ok() {
                stack.push(token.parse::<f64>().unwrap());
            } else if is_operator(token) {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                match token.as_str() {
                    "+" => stack.push(basic::add(a, b)),
                    "-" => stack.push(basic::subtract(b, a)),
                    "*" => stack.push(basic::multiply(a, b)),
                    "/" => stack.push(basic::divide(b, a)?),
                    "^" => stack.push(basic::power(b, a)),
                    "%" => stack.push(basic::modulus(b, a)),
                    _ => return Err(format!("Invalid operator {}", token)),
                }
            } else if is_function(token) {
                let a = stack.pop().unwrap();

                match token.as_str() {
                    "sin" => stack.push(basic::sine(a)),
                    "cos" => stack.push(basic::cosine(a)),
                    "tan" => stack.push(basic::tangent(a)),
                    "ln" => stack.push(basic::ln(a)),
                    "lg" => stack.push(basic::lg(a)),
                    "ceil" => stack.push(basic::ceil(a)),
                    "floor" => stack.push(basic::floor(a)),
                    "abs" => stack.push(basic::abs(a)),
                    "round" => stack.push(basic::round(a)),
                    "sqrt" => stack.push(basic::sqrt(a)),
                    _ => return Err(format!("Invalid function {}", token)),
                }
            } else {
                return Err(format!("Invalid expression"));
            }
        }
        println!("Result: {}", stack.pop().unwrap());
        Ok(())
    }
}
