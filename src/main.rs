#![allow(dead_code)]

extern crate num_rational;

use std::fs;

mod tokeniser;
mod interpreter;
mod ast;
mod compiler;
mod parser;

type Fraction = num_rational::BigRational;

fn main() {
    
    let src = fs::read_to_string("examples/tmp.mx").expect("File io error");
    let tokens = tokeniser::tokenise(&src);
    println!("{:#?}", tokens);
    let program = parser::parse(tokens).expect("Failed to parse");
    println!("Parsed: {:#?}", program);
    let func = program.functions[0].compile();
    println!("Code: {:#?}", func);

}