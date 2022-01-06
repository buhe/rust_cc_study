pub mod lexer;
pub mod parser;
pub mod ast;
pub mod ir;
pub mod codegen;
pub mod regeister;
pub mod symbols;
pub mod ir_handle;

use lexer::tokenize;
use parser::parsing;
use std::io::{Result, Write};

use crate::ir_handle::op;

pub fn run(path: String, output: &mut impl Write) -> Result<()> {
  let t = tokenize(path);
  println!("Tokens: {:#?}", t);
  let mut p = parsing(&t);
  println!("Prog: {:#?}", &p.0);
  let p_ir = ir::ast2ir(&p.0, &mut p.1);
  println!("IR Prog: {:#?}", &p_ir.0);
  let ir = op(&p_ir.0, &mut p.1);
  codegen::write_asm(&ir,&mut p.1, output)
}
