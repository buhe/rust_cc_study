use crate::ir::{IrStmt, IrProg};

pub fn dataflow(p: &IrProg) -> &IrProg {
    p
}

pub struct BasicBlock {
   pub stmts: Vec<IrStmt>,
   pub edge: Option<Box<BasicBlock>>,  
}
