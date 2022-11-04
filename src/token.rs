#![deny(clippy::all)]

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Lparen,
    Rparen,
    Number(f64)
}