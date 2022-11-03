#![deny(clippy::all)]

#[derive(Debug)]
pub enum TokenType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Lparen,
    Rparen,
    Number
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub number_value: f64,
}

impl Token {
    pub fn to_string(&self) -> String {
        format!("{:?}", &self)
    }

    pub fn new(token_type: TokenType) -> Self {
        let instance = Token {token_type: token_type, number_value: 0.0};
        instance
    }

    pub fn set_number_value(&mut self, number_value: f64) {
        self.number_value = number_value;
    }
}