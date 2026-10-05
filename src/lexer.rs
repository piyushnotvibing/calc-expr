use std::f64::consts;

use crate::errors::LexerError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),
    Identity(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
}

const FUNC_CHARS: [char; 29] = [
    '0', '1', '2', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const FLOAT_CHARS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
const EPSILON: char = '#';

pub fn tokenize(mut expr: String) -> Result<Vec<Token>, LexerError> {
    strip_whitespaces(&mut expr);
    let mut tokens: Vec<Token> = Vec::new();
    let mut literal: String = String::new();
    let mut identity: String = String::new();
    let mut characters = expr.chars().peekable();

    while let Some(c) = characters.next() {
        let next_char = characters
            .peek()
            .copied()
            .unwrap_or(EPSILON)
            .to_ascii_lowercase();
        match c {
            '+' => push_token(
                &Token::Plus,
                &mut tokens,
                &mut literal,
                &mut identity,
                next_char,
            )?,
            '-' => push_token(
                &Token::Minus,
                &mut tokens,
                &mut literal,
                &mut identity,
                next_char,
            )?,
            '*' => push_token(
                &Token::Star,
                &mut tokens,
                &mut literal,
                &mut identity,
                next_char,
            )?,
            '/' => push_token(
                &Token::Slash,
                &mut tokens,
                &mut literal,
                &mut identity,
                next_char,
            )?,
            '^' => push_token(
                &Token::Caret,
                &mut tokens,
                &mut literal,
                &mut identity,
                next_char,
            )?,
            '(' => push_token(
                &Token::LParen,
                &mut tokens,
                &mut literal,
                &mut identity,
                next_char,
            )?,
            ')' => push_token(
                &Token::RParen,
                &mut tokens,
                &mut literal,
                &mut identity,
                next_char,
            )?,
            ',' => push_token(
                &Token::Comma,
                &mut tokens,
                &mut literal,
                &mut identity,
                next_char,
            )?,
            _ => {
                // Since .x can be parsed by rust into 0.x, we will allow it
                if c == '.' {
                    let period_invalid = period_already_in_literal(&literal)
                        || a_number_does_not_follow_period(&next_char);
                    if period_invalid {
                        return Err(LexerError::UnexpectedPeriod);
                    }
                }

                if !FLOAT_CHARS.contains(&c) && !FUNC_CHARS.contains(&c.to_ascii_lowercase()) {
                    return Err(LexerError::InvalidCharacter(c));
                }

                let c = c.to_ascii_lowercase();
                // numbers like 0, 1 and 2 appear in numbers as well as functions log10 and log2
                if FLOAT_CHARS.contains(&c) && FUNC_CHARS.contains(&c) {
                    if identity.is_empty() {
                        literal.push(c);
                    } else {
                        identity.push(c);
                    }
                } else if FLOAT_CHARS.contains(&c) {
                    literal.push(c);
                } else if FUNC_CHARS.contains(&c) {
                    // implicit multiplication 3: 5sin(pi/2) => 5 * sin(pi/2)
                    if !literal.is_empty() && identity.is_empty() {
                        parse_and_push_num_token(&mut literal, &mut tokens)?;
                        literal.clear();
                        tokens.push(Token::Star);
                    }
                    identity.push(c);
                }
            }
        }
    }

    if !literal.is_empty() {
        parse_and_push_num_token(&literal, &mut tokens)?;
    }
    if !identity.is_empty() {
        parse_and_push_identity_token(&identity, &mut tokens)?;
    }

    Ok(tokens)
}

fn push_token(
    token: &Token,
    tokens: &mut Vec<Token>,
    literal: &mut String,
    identity: &mut String,
    next_char: char,
) -> Result<(), LexerError> {
    let literal_exists = !literal.is_empty();
    let identity_exists = !identity.is_empty();

    if literal_exists {
        parse_and_push_num_token(literal, tokens)?;
        literal.clear();
    }

    if identity_exists {
        parse_and_push_identity_token(identity, tokens)?;
        identity.clear();
    }

    // implicit multiplication 1: 5(3 + 4) => 5 * (3 + 4) | (5 / 4)(3 + 5) => (5 / 4) * (3 + 5)
    if *token == Token::LParen {
        if literal_exists {
            tokens.push(Token::Star);
        } else if let Some(prev_token) = tokens.last() {
            if *prev_token == Token::RParen {
                tokens.push(Token::Star);
            }
        }
    }

    tokens.push(token.clone());

    // implicit multiplication 4: (3 + 4)5 => (3 + 4) * 5
    if *token == Token::RParen {
        let next_char_is_literal =
            FLOAT_CHARS.contains(&next_char) && next_char != '-' && next_char != '.';
        let next_char_is_identity = FUNC_CHARS.contains(&next_char)
            && next_char != '0'
            && next_char != '1'
            && next_char != '2';
        if next_char_is_literal || next_char_is_identity {
            tokens.push(Token::Star);
        }
    }

    Ok(())
}

fn strip_whitespaces(expr: &mut String) {
    let cleaned = expr
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    *expr = cleaned;
}

#[inline(always)]
fn period_already_in_literal(literal: &str) -> bool {
    !literal.is_empty() && literal.contains('.')
}

#[inline(always)]
fn a_number_does_not_follow_period(next_char: &char) -> bool {
    !['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(next_char)
}

fn parse_and_push_num_token(literal: &str, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
    if !literal.is_empty() {
        let num = match literal.parse() {
            Ok(n) => n,
            Err(e) => return Err(LexerError::InvalidFloat(e)),
        };
        tokens.push(Token::Num(num));
    }
    Ok(())
}

fn parse_and_push_identity_token(
    identity: &str,
    tokens: &mut Vec<Token>,
) -> Result<(), LexerError> {
    if identity.is_empty() {
        return Err(LexerError::Wtf);
    }
    match identity {
        "pi" => tokens.push(Token::Num(consts::PI)),
        "e" => tokens.push(Token::Num(consts::E)),
        "tau" => tokens.push(Token::Num(consts::TAU)),
        _ => tokens.push(Token::Identity(identity.to_string())),
    }
    Ok(())
}
