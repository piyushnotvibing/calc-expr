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
    P = U P'
    U = - M | M
    P' = * U P' | / U P' | epsilon
    M = A M'
    M' = ^ U | epsilon
    A = T | N | R | (S) | Function(I)
    I = SI'
    I' = , SI' | epsilon
    T = 0 | 1 Q | 2 Q | ... | 9 Q
    N = - 1Q | - 2 Q | ... | - 9 Q
    R = T.TQ | N.TQ
    Q = TQ | epsilon
*/

use crate::{
    errors::ParserError::{self, InvalidExpr},
    lexer::Token,
};

#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Option<Box<Expr>>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
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
        let tok = self.tokens.get(self.pos).cloned();
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
                    lhs = Expr::Sub(Box::new(lhs), Some(Box::new(expr)));
                }
                _ => break,
            }
        }

        Ok(lhs)
    }

    // P = U P'
    fn parse_p(&mut self) -> Result<Expr, ParserError> {
        let expr = self.parse_u()?;
        self.parse_p_prime(expr)
    }

    // P' = * U P' | / U P' | epsilon
    fn parse_p_prime(&mut self, lhs: Expr) -> Result<Expr, ParserError> {
        let mut lhs = lhs;
        while let Some(token) = self.peek() {
            match token {
                Token::Star => {
                    let _ = self.advance();
                    let expr = self.parse_u()?;
                    lhs = Expr::Mul(Box::new(lhs), Box::new(expr))
                }
                Token::Slash => {
                    let _ = self.advance();
                    let expr = self.parse_u()?;
                    lhs = Expr::Div(Box::new(lhs), Box::new(expr))
                }
                _ => break,
            }
        }

        Ok(lhs)
    }
    
    // U = - M | M
    fn parse_u(&mut self) -> Result<Expr, ParserError> {
        if let Some(token) = self.peek() && *token == Token::Minus {
            let _ = self.advance();
            let expr = self.parse_m()?;
            return Ok(Expr::Sub(Box::new(expr), None))
        }

        self.parse_m()
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
                return Ok(lhs);
            }

            let _ = self.advance();
            let expr = self.parse_u()?;
            lhs = Expr::Pow(Box::new(lhs), Box::new(expr));
        }

        Ok(lhs)
    }

    // A = T | R | (S) | - (S) | Function(I) | - Function(I)
    fn parse_a(&mut self) -> Result<Expr, ParserError> {
        match self.peek() {
            // If A = T | R, then return Expr::Num(n).
            Some(Token::Num(n)) => {
                let n = *n;
                let _ = self.advance();
                return Ok(Expr::Num(n));
            }
            // If A = (S), return wtv expression S computes.
            Some(Token::LParen) => {
                let _ = self.advance();
                let expr = self.parse_s()?;
                if let Some(token) = self.peek() {
                    if *token == Token::RParen {
                        let _ = self.advance();
                        return Ok(expr);
                    } else {
                        // If the token after '(S' is not ')', then the expression is invalid.
                        return Err(InvalidExpr(Some(token.clone()), self.pos, "A"));
                    }
                }
                // If there are no tokens left after '(S', then the expression is invalid.
                return Err(InvalidExpr(None, self.pos, "A"));
            }
            // If A = Function(I), return
            Some(Token::Identity(f)) => {
                let function = f.clone();
                let _ = self.advance();
                if let Some(token) = self.peek() {
                    if *token == Token::LParen {
                        let _ = self.advance();
                        let args = self.parse_i()?;
                        if let Some(token) = self.peek() {
                            if *token == Token::RParen {
                                let _ = self.advance();
                                return Ok(Expr::Call(function, args));
                            } else {
                                return Err(InvalidExpr(Some(token.clone()), self.pos, "A"));
                            }
                        }
                    } else {
                        return Err(InvalidExpr(Some(token.clone()), self.pos, "A"));
                    }
                }

                Err(InvalidExpr(Some(Token::Identity(function)), self.pos, "A"))
            }

            // Any token other than a number or a left paranthesis is an invalid expression.
            Some(t) => return Err(ParserError::InvalidExpr(Some(t.clone()), self.pos, "A")),
            _ => return Err(ParserError::InvalidExpr(None, self.pos, "A")),
        }
    }

    fn parse_i(&mut self) -> Result<Vec<Expr>, ParserError> {
        let expr = self.parse_s()?;
        self.parse_i_prime(expr)
    }

    fn parse_i_prime(&mut self, first_arg: Expr) -> Result<Vec<Expr>, ParserError> {
        let mut args: Vec<Expr> = Vec::new();
        args.push(first_arg);

        while let Some(token) = self.peek() {
            match token {
                Token::Comma => {
                    let _ = self.advance();
                    let expr = self.parse_s()?;
                    args.push(expr);
                }
                _ => break,
            }
        }

        Ok(args)
    }

    // fn parse_identity(identity: &str) -> Result<Expr, ParserError> {

    // }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParserError> {
    let mut parser = Parser::from(tokens);
    let expr = parser.parse_s()?;

    if parser.pos != parser.tokens.len() {
        return Err(InvalidExpr(None, parser.pos, "S"));
    }
    Ok(expr)
}
