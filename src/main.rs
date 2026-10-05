use std::env;

mod errors;

mod lexer;
use lexer::tokenize;

mod parser;
use parser::parse;

mod eval;
use eval::eval;

mod functions;
use crate::functions::Registry;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: calc-expr ARITHMETIC_EXPR");
    }

    let tokens = match tokenize(args[1].clone()) {
        Ok(t) => t,
        Err(e) => panic!("{e}"),
    };

    println!("{:?}", tokens);

    let expr = match parse(tokens) {
        Ok(expr) => expr,
        Err(e) => panic!("{e}"),
    };

    println!("{:?}", expr);

    let registry = Registry::build();
    let res = match eval(&expr, &registry) {
        Ok(value) => value,
        Err(e) => panic!("{e}"),
    };

    println!("{res}")
}
