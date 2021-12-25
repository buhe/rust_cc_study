pub mod lexer;
pub mod parser;
pub mod ast;
pub mod ir;
pub mod codegen;
pub mod regeister;
pub mod check;
pub mod symbols;

use lexer::tokenize;
use parser::parsing;
use std::io::{Result, Write};

pub fn run(path: String, output: &mut impl Write) -> Result<()> {
  let t = tokenize(path);
  println!("Tokens: {:#?}", t);
   let p = parsing(&t);
     println!("Prog: {:#?}", &p.0);
   //   let p = check::check(&p.0);
     let p = ir::ast2ir(&p.0, &p.1);
     println!("IR Prog: {:#?}", &p);
     codegen::write_asm(&p, output)
  // Ok(())
}
