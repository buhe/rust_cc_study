use crate::{ir::*, symbols::{SymTab}};
use std::io::{Result, Write};

pub fn write_asm(p: &IrProg, bl: &mut BranchLabel ,table: &mut SymTab, w: &mut impl Write) -> Result<()> {
  let f = &p.func;
  
  writeln!(w, ".global {}", f.name)?;
  writeln!(w, "{}:", f.name)?;
  for s in &f.stmts {
    match s {
      IrStmt::Neg(t1, t2) => {
        writeln!(w, "  sub {}, x0, {}", t2,t1)?;
      }
      IrStmt::Ldc(x, reg) => {
        writeln!(w, "  addi {}, x0, {}", reg, x)?; //todo
      }
      IrStmt::Ret(t) => {
        writeln!(w, "  addi a0, {}, 0", t)?;
        writeln!(w, "  jalr x0, x1, 0")?;
      }
      IrStmt::Add(t1, t2, t) => {
        writeln!(w, "  add {} ,{} ,{}", t, t2, t1)?;
      }
      IrStmt::Sub(t1, t2, t) => {
        writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
      }
      IrStmt::Div(t1, t2, t) => {
        writeln!(w, "  div {} ,{} ,{}", t, t2, t1)?; //todo
      }
      IrStmt::Mod(t1, t2, t) => {
        writeln!(w, "  mod {} ,{} ,{}", t, t2, t1)?; // todo
      }
      IrStmt::Mul(t1, t2, t) => {
        writeln!(w, "  mul {} ,{} ,{}", t, t2, t1)?; //todo
      }
        IrStmt::Or(t1, t2, t)=> {
          // or t3,t1,t2 ; snez t3,t3
          writeln!(w, "  or {} ,{} ,{}", t, t2, t1)?;
          writeln!(w, "  sltu {} ,x0 ,{}", t, t)?;
        }
        IrStmt::And(t1, t2, t) => {
          // snez d, s1; sub d, zero, d; and d, d, s2; snez d, d;
          writeln!(w, "  sltu {} ,x0 ,{}", t, t2)?;
          writeln!(w, "  sub {} ,zero ,{}", t, t)?;
          writeln!(w, "  and {} ,{} ,{}", t, t, t1)?;
          writeln!(w, "  sltu {} ,x0 ,{}", t, t)?;
        }
        IrStmt::Equal(t1, t2, t, t3, t4) => {
          // seqz t1,t2	Set EQual to Zero : if t2 == 0 then set t1 to 1 else 0
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          writeln!(w, "  addi {} ,x0 ,1", t4)?;
          writeln!(w, "  sltu {} ,{} ,{}", t3, t, t4)?;
        }
        IrStmt::NotEqual(t1, t2, t, t3) => {
          // snez t1,t2	Set Not Equal to Zero : if t2 != 0 then set t1 to 1 else 0
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          writeln!(w, "  sltu {} ,x0 ,{}", t3, t)?;
        }
        IrStmt::Lt(t1, t2, t) => {
          writeln!(w, "  slt {} ,{} ,{}", t, t2, t1)?;
        }
        IrStmt::Let(t1, t2, t, t3, t4, t5, t6) => {
          // eq
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          writeln!(w, "  addi {} ,x0 ,1", t6)?;
          writeln!(w, "  sltu {} ,{} ,{}", t3, t, t6)?;
          // lt
          writeln!(w, "  slt {} ,{} ,{}", t4, t2, t1)?;
          // or
          writeln!(w, "  or {} ,{} ,{}", t5, t3, t4)?;
          writeln!(w, "  sltu {} ,x0 ,{}", t5, t5)?;
        }
        IrStmt::Gt(t1, t2, t) => {// todo slt
          writeln!(w, "  sgt {} ,{} ,{}", t, t2, t1)?;
        }
        IrStmt::Get(t1, t2, t, t3, t4, t5) => {// todo slt
          // eq
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          writeln!(w, "  seqz {} ,{}", t3, t)?;
          // gt
          writeln!(w, "  sgt {} ,{} ,{}", t4, t2, t1)?;
          // or
          writeln!(w, "  or {} ,{} ,{}", t5, t3, t4)?;
          writeln!(w, "  snez {} ,{}", t5, t5)?;
        }
        IrStmt::Assign(scope, id,t2) => {
          let s = table.get(scope, id);
          writeln!(w, "  addi {} ,{}, 0", s.reg.as_ref().unwrap(), t2)?;
        },
        IrStmt::Ref(_scope,_id) => {
          // use
        },
        IrStmt::Beq(t) => {
          let l = bl.label();
          writeln!(w, "  beq {} ,x0 ,{}", t, l.name)?;
        },
        IrStmt::Jmp => {
          let l = bl.label();
          writeln!(w, "  jal x0, {}", l.name)?;
        },
        IrStmt::Label(label) => {
          writeln!(w, "{}:", label)?;
        },
    }
  }
  Ok(())
}
