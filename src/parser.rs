/* Old Grammar:
    // S' = S
    S = A S'
    S' = + A S' | - A S' | * A S' | / A S' | ^ A S' | epsilon
    A = T | R | (S)
    T = 0 | 1Q | 2Q | ... | 9Q
    R = T.TQ
    Q = TQ | epsilon
*/
/* New Grammar:
    S = P S'
    S' = + P S' | - P S' | epsilon
    P = M P'
    P' = * M P' | / M P' | epsilon
    M = A M'
    M' = ^ M | epsilon
    A = T | R | (S)
    T = 0 | 1Q | 2Q | ... | 9Q
    R = T.TQ
    Q = TQ | epsilon
*/

use crate::{
    errors::ParserError::{self, InvalidExpr},
    lexer::Token,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn from(tokens: Vec<Token>) -> Parser {
        let pos = 0;
        Self { tokens, pos }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).copied();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    // S = P S'
    fn parse_s(&mut self) -> Result<Expr, ParserError> {
        let expr = self.parse_p()?;
        self.parse_s_prime(expr)
    }

    // S' = + P S' | - P S' | epsilon
    fn parse_s_prime(&mut self, lhs: Expr) -> Result<Expr, ParserError> {
        let mut lhs = lhs;
        while let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    let _ = self.advance();
                    let expr = self.parse_p()?;
                    lhs = Expr::Add(Box::new(lhs), Box::new(expr));
                }
                Token::Minus => {
                    let _ = self.advance();
                    let expr = self.parse_p()?;
                    lhs = Expr::Sub(Box::new(lhs), Box::new(expr));
                }
                _ => break,
            }
        }

        Ok(lhs)
    }

    // P = M P'
    fn parse_p(&mut self) -> Result<Expr, ParserError> {
        let expr = self.parse_m()?;
        self.parse_p_prime(expr)
    }

    // P' = * M P' | / M P' | epsilon
    fn parse_p_prime(&mut self, lhs: Expr) -> Result<Expr, ParserError> {
        let mut lhs = lhs;
        while let Some(token) = self.peek() {
            match token {
                Token::Star => {
                    let _ = self.advance();
                    let expr = self.parse_m()?;
                    lhs = Expr::Mul(Box::new(lhs), Box::new(expr))
                },
                Token::Slash => {
                    let _ = self.advance();
                    let expr = self.parse_m()?;
                    lhs = Expr::Div(Box::new(lhs), Box::new(expr))
                },
                _ => break,
            }
        }

        Ok(lhs)
    }

    // M = A M'
    fn parse_m(&mut self) -> Result<Expr, ParserError> {
        let expr = self.parse_a()?;
        self.parse_m_prime(expr)
    }

    // M' = ^ M | epsilon
    fn parse_m_prime(&mut self, lhs: Expr) -> Result<Expr, ParserError> {
        let mut lhs = lhs;
        if let Some(token) = self.peek() {
            if *token != Token::Caret {
                return Ok(lhs)
            }

            let _ = self.advance();
            let expr = self.parse_m()?;
            lhs = Expr::Pow(Box::new(lhs), Box::new(expr));
        }

        Ok(lhs)
    }

    // A = T | R | (S)
    fn parse_a(&mut self) -> Result<Expr, ParserError> {
        match self.peek() {
            Some(&token) => {
                match token {
                    // If A = T | R, then return Expr::Num(n).
                    Token::Num(n) => {
                        let _ = self.advance();
                        Ok(Expr::Num(n))
                    }
                    // If A = (S), return wtv expression S computes.
                    Token::LParen => {
                        let _ = self.advance();
                        let expr = self.parse_s()?;
                        if let Some(&token) = self.peek() {
                            if token == Token::RParen {
                                let _ = self.advance();
                                return Ok(expr);
                            } else {
                                // If the token after '(S' is not ')', then the expression is invalid.
                                return Err(InvalidExpr(Some(token), self.pos, "A"));
                            }
                        }
                        // If there are no tokens left after '(S', then the expression is invalid.
                        Err(InvalidExpr(None, self.pos, "A"))
                    }
                    // Any token other than a number or a left paranthesis is an invalid expression.
                    _ => Err(ParserError::InvalidExpr(Some(token), self.pos, "A")),
                }
            }
            // If there is no token at current pos, then its invalid because A cannot be epsilon.
            None => Err(ParserError::InvalidExpr(None, self.pos, "A")),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParserError> {
    let mut parser = Parser::from(tokens);
    parser.parse_s()
}
