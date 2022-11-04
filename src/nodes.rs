use crate::token::Token;

pub enum AstNode<'a> {
    NumberNode(Token),
    BinaryOperationNode(&'a AstNode<'a>, Token, &'a AstNode<'a>),
    None
}
impl<'a> Clone for AstNode<'a> {
    fn clone(&self) -> Self {
        match self {
            Self::NumberNode(arg0) => Self::NumberNode(*arg0),
            Self::BinaryOperationNode(arg0, arg1, arg2) => Self::BinaryOperationNode(arg0.clone(), *arg1, arg2.clone()),
            Self::None => Self::None,
        }
    }

    fn clone_from(&mut self, source: &Self)
    {
        *self = source.clone()
    }
}

impl<'a> AstNode<'a> {
    pub fn copy(&self) -> Self {
        self.clone()
    }
}