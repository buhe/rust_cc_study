pub mod lexer;
pub mod parser;
pub mod ast;
pub mod ir;
pub mod codegen;

use lexer::tokenize;
use parser::parsing;
use std::io::{Result, Write};

pub fn run(path: String, output: &mut impl Write) -> Result<()> {
  let t = tokenize(path);
  // println!("Tokens: {:#?}", t);
   let p = parsing(&t);
    //  println!("Prog: {:#?}", &p);
     let p = ir::ast2ir(&p);
    //  println!("IR Prog: {:#?}", &p);
     codegen::write_asm(&p, output)
}
