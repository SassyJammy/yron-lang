use crate::error::Error;
use crate::position::Position;
use crate::token::{Token, TokenType};

pub struct Lexer {
    pub text: String,
    pub file_name: String,
    position: Position,
    current_char: char,
}

impl Lexer {
    pub fn new(text: String, file_name: String) -> Self {
        let mut instance = Lexer {
            text: text.clone(),
            position: Position::new(-1, 0, -1, file_name.clone(), text.clone()),
            current_char: ' ',
            file_name,
        };
        instance.advance();

        instance
    }

    fn index_string(&self, string: &String, index: i64) -> char {
        let byte = string.as_bytes()[index as usize];
        byte as char
    }

    pub fn advance(&mut self) {
        self.position.advance(self.current_char);

        if self.position.index < self.text.len() as i64 {
            self.current_char = self.index_string(&self.text, self.position.index);
        }
    }

    fn generate_number(&mut self) -> Token {
        let mut number_string = "".to_string();

        // while position is less then text length and
        // current_char is a digit or '.'
        while self.position.index < self.text.len() as i64
            && (self.current_char.is_digit(10) || self.current_char == '.')
        {
            number_string += format!("{}", self.current_char).as_str();
            self.advance();
        }

        let result = number_string.parse::<f64>().unwrap();
        let mut tok = Token::new(TokenType::Number);
        tok.set_number_value(result);

        tok
    }

    pub fn generate_tokens(mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = vec![];

        while self.position.index < self.text.len() as i64 {
            if "\n\t ".contains(self.current_char) {
                self.advance();
                continue;
            }

            if self.current_char.is_digit(10) {
                tokens.push(self.generate_number());
                continue;
            }

            match self.current_char {
                '+' => tokens.push(Token::new(TokenType::Plus)),
                '-' => tokens.push(Token::new(TokenType::Minus)),
                '*' => tokens.push(Token::new(TokenType::Multiply)),
                '/' => tokens.push(Token::new(TokenType::Divide)),
                '(' => tokens.push(Token::new(TokenType::Lparen)),
                ')' => tokens.push(Token::new(TokenType::Rparen)),
                _ => {
                    let current_char = self.current_char;
                    let start_position: Position = self.position.copy();
                    self.advance();
                    let end_position: Position = self.position.copy();
                    return Err(Error::new_illegal_char(
                        start_position,
                        end_position,
                        format!("Unknown character '{}'", current_char),
                    ));
                }
            }

            self.advance();
        }

        Ok(tokens)
    }
}
