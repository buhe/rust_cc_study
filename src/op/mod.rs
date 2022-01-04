use crate::ir::IrProg;

mod dataflow;

pub fn op(p: &IrProg) -> IrProg {
    dataflow::dataflow(p)
}