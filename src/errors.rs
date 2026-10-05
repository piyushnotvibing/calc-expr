use std::{error::Error, fmt::Display, num::ParseFloatError};

use crate::lexer::Token;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedPeriod,
    InvalidCharacter(char),
    InvalidFloat(ParseFloatError),
    Wtf
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedPeriod => write!(f, "lexer: unexpected period"),
            LexerError::InvalidCharacter(c) => write!(f, "lexer: invalid character: {c}"),
            LexerError::InvalidFloat(e) => write!(f, "lexer: invalid float: {e}"),
            LexerError::Wtf => write!(f, "lexer: identity is empty? wtf?"),
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
    NaN,
    IncorrectNumOfArgs(String),
    InvalidFunction(String),
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::DivBy0 => write!(f, "eval: invalid operation attemped => divide by zero",),
            EvalError::NaN => write!(
                f,
                "eval: invalid operation attemped => not a number",
            ),
            EvalError::IncorrectNumOfArgs(func) => write!(
                f,
                "eval: invalid operation attemped => incorrect number of arguments provided to {func}",
            ),
            EvalError::InvalidFunction(func) => write!(f, "eval: invalid operation attemped => invalid function or function not implemented: {func}",),
        }
    }
}

impl Error for EvalError {}
