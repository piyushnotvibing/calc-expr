use std::{error::Error, fmt::Display, num::ParseFloatError};

use crate::lexer::Token;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedPeriod,
    InvalidCharacter(char),
    InvalidFloat(ParseFloatError),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedPeriod => write!(f, "lexer: unexpected period"),
            LexerError::InvalidCharacter(c) => write!(f, "lexer: invalid character: {c}"),
            LexerError::InvalidFloat(e) => write!(f, "lexer: invalid float: {e}"),
        }
    }
}

impl Error for LexerError {}

#[derive(Debug)]
pub enum ParserError {
    InvalidExpr(Option<Token>, usize, &'static str),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::InvalidExpr(t, i, s) => write!(
                f,
                "parser: invalid arithmetic expression => token {:?} at index {i} | Non-terminal where error was encountered: {s}",
                t
            ),
        }
    }
}

impl Error for ParserError {}

#[derive(Debug)]
pub enum EvalError {
    DivBy0,
    NaN
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::DivBy0 => write!(f, "eval: invalid operation attemped => divide by zero",),
            EvalError::NaN => write!(f, "eval: invalid operation attemped => raising a negative number by a fraction",),
        }
    }
}

impl Error for EvalError {}
