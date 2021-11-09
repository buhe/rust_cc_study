pub mod lexer;

use lexer::tokenize;
use std::io::{Result, Write};

pub fn run(path: String, _output: &mut impl Write) -> Result<()> {
  let t = tokenize(path);
  println!("Tokens: {:#?}", t);
  Ok(())
}
