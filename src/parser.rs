use crate::{
    nodes::AstNode,
    token::Token,
};

struct Parser {
    tokens: &'static Vec<Token>,
    current_token: &'static Token,
    token_index: i64,
}
impl Parser {
    pub fn new(tokens: &'static Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            current_token: &tokens[0],
            token_index: -1,
        };
        parser.advance();

        parser
    }

    pub fn advance(&mut self) {
        self.token_index += 1;
        if self.token_index < self.tokens.len() as i64 {
            self.current_token = &self.tokens[self.token_index as usize];
        }
    }

    fn factor(&mut self) -> AstNode {
        let token = &mut self.current_token.clone();

        if let Token::Number(value) = token {
            self.advance();
            return AstNode::NumberNode(*token);
        }

        AstNode::None
    }

    fn term(&mut self) -> AstNode {
        let left = self.factor().copy();
        let mut current_tok = self.current_token;

        while vec![Token::Multiply, Token::Divide].contains(&self.current_token) {
            let operator_token = self.current_token.clone();
            let right = self.factor();
            left = AstNode::BinaryOperationNode(&left.copy(), operator_token, &right.copy());
        }

        left
    }

    fn expression(&mut self) {}
}
