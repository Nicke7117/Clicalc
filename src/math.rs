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
        "+" => true,
        "-" => true,
        "*" => true,
        "/" => true,
        "^" => true,
        "%" => true,
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
        self.convert_to_reverse_polish_notation();
        self.solve_reverse_polish_notation();
        println!("{:?}", self.reverse_polish_notation);
    }

    fn split_into_tokens(&mut self) {
        let mut token = String::new();
        for (i, c) in self.expression.chars().enumerate() {
            if c.is_numeric() {
                token.push(c);
                if i == self.expression.len() - 1 {
                    self.tokens.push(token.clone());
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
    fn convert_to_reverse_polish_notation(&mut self) {
        let mut queue: Vec<String> = Vec::new();
        let mut stack: Vec<String> = Vec::new();
        let precedence =
            HashMap::from([("+", 1), ("-", 1), ("*", 2), ("/", 2), ("^", 3), ("%", 3)]);
        for token in self.tokens.iter() {
            if token.parse::<f64>().is_ok() {
                queue.push(token.to_string())
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
            } else if is_operator(token) {
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
    fn solve_reverse_polish_notation(&mut self) {
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
                    "/" => stack.push(basic::divide(b, a)),
                    "^" => stack.push(basic::power(b, a)),
                    "%" => stack.push(basic::modulus(b, a)),
                    _ => panic!("Invalid operator: {}", token),
                }
            }
        }
        println!("{}", stack.pop().unwrap());
    }
}
