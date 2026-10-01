use crate::errors::LexerError;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    Num(f64),
    // Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}

const FLOAT_CHARS: [char; 12] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '-'];
const EPSILON: char = '#';

pub fn tokenize(mut expr: String) -> Result<Vec<Token>, LexerError> {
    strip_whitespaces(&mut expr);
    let mut tokens: Vec<Token> = Vec::new();
    let mut literal: String = String::new();
    let mut characters = expr.chars().peekable();

    while let Some(c) = characters.next() {
        let next = characters.peek();
        let next_char = next.copied().unwrap_or(EPSILON);
        match c {
            '+' => push_token(Token::Plus, &mut tokens, &mut literal, next_char)?,
            '-' => push_token(Token::Minus, &mut tokens, &mut literal, next_char)?,
            '*' => push_token(Token::Star, &mut tokens, &mut literal, next_char)?,
            '/' => push_token(Token::Slash, &mut tokens, &mut literal, next_char)?,
            '^' => push_token(Token::Caret, &mut tokens, &mut literal, next_char)?,
            '(' => push_token(Token::LParen, &mut tokens, &mut literal, next_char)?,
            ')' => push_token(Token::RParen, &mut tokens, &mut literal, next_char)?,
            _ => {
                // Since .x can be parsed by rust into 0.x, we will allow it
                if c == '.' {
                    if let Some(next_char) = next {
                        let period_invalid = period_already_in_literal(&literal)
                            || a_number_does_not_follow_period(next_char);
                        if period_invalid {
                            return Err(LexerError::UnexpectedPeriod);
                        }
                    } else {
                        return Err(LexerError::UnexpectedPeriod);
                    }
                }
                if !FLOAT_CHARS.contains(&c) {
                    return Err(LexerError::InvalidCharacter(c));
                }
                literal.push(c);
            }
        }
    }

    parse_and_push_num_token(&literal, &mut tokens)?;
    Ok(tokens)
}

fn push_token(
    token: Token,
    tokens: &mut Vec<Token>,
    literal: &mut String,
    next_char: char,
) -> Result<(), LexerError> {
    let literal_exists = !literal.is_empty();
    if literal_exists {
        parse_and_push_num_token(literal, tokens)?;
        literal.clear();
    }

    // -3+4 should get tokenized to [Num(-3), Plus, Num(4)]
    if token == Token::Minus && !literal_exists && a_number_follows_the_minus(&next_char) {
        literal.push('-');
        return Ok(());
    }

    // implicit multiplication 1: 5(3 + 4) => 5 * (3 + 4) | (5 / 4)(3 + 5) => (5 / 4) * (3 + 5)
    if token == Token::LParen {
        if literal_exists {
            tokens.push(Token::Star);
        } else if let Some(prev_token) = tokens.last() {
            if *prev_token == Token::RParen {
                tokens.push(Token::Star);
            }
        }
    }

    tokens.push(token);

    // implicit multiplication 2: (3 + 4)5 => (3 + 4) * 5
    if token == Token::RParen {
        if FLOAT_CHARS.contains(&next_char) {
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

#[inline(always)]
fn a_number_follows_the_minus(next_char: &char) -> bool {
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(next_char)
}

fn parse_and_push_num_token(literal: &str, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
    if !literal.is_empty() {
        // parse() handles negative numbers too, so "-34" becomes -34.
        let num = match literal.parse() {
            Ok(n) => n,
            Err(e) => return Err(LexerError::InvalidFloat(e)),
        };
        tokens.push(Token::Num(num));
    }
    Ok(())
}
