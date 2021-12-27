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
   let mut p = parsing(&t);
     println!("Prog: {:#?}", &p.0);
   //   let p = check::check(&p.0);
     let p_ir = ir::ast2ir(&p.0, &mut p.1);
     println!("IR Prog: {:#?}", &p_ir);
     codegen::write_asm(&p_ir,&mut p.1, output)
  // Ok(())
}
