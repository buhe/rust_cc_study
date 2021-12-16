use crate::{ir::*, regeister::Regeister};
use std::io::{Result, Write};

pub fn write_asm(p: &IrProg, w: &mut impl Write) -> Result<()> {
  let f = &p.func;
  let mut r = Regeister::init();
  writeln!(w, ".global {}", f.name)?;
  writeln!(w, "{}:", f.name)?;
  for s in &f.stmts {
    match s {
      IrStmt::Neg => {
        let t1 = r.near();
        let t2 = r.eat();
        writeln!(w, "  neg {} , {}", t2,t1)?;
        // writeln!(w, "  sw {}, 0(sp)", t2)?;
      }
      IrStmt::Ldc(x) => {
        let t = r.eat();
        writeln!(w, "  li {}, {}", t, x)?;
        // writeln!(w, "  sw {}, -8(sp)", t)?;
        // writeln!(w, "  add sp, sp, -8")?;
      }
      IrStmt::Ret => {
        writeln!(w, "  lw a0, 0(sp)")?;
        // writeln!(w, "  add sp, sp, 8")?; 
        writeln!(w, "  ret")?;
      }
      IrStmt::Add => {
        let t1 = r.near();
        let t2 = r.near();
        let t = r.eat();
        writeln!(w, "  add, {} ,{} ,{}", t, t1, t2)?;
      }
      IrStmt::Sub => {
        let t1 = r.near();
        let t2 = r.near();
        let t = r.eat();
        writeln!(w, "  sub, {} ,{} ,{}", t, t1, t2)?;
      }
      IrStmt::Div => {
        let t1 = r.near();
        let t2 = r.near();
        let t = r.eat();
        writeln!(w, "  div, {} ,{} ,{}", t, t1, t2)?;
      }
      IrStmt::Mod => {
        let t1 = r.near();
        let t2 = r.near();
        let t = r.eat();
        writeln!(w, "  mod, {} ,{} ,{}", t, t1, t2)?;
      }
      IrStmt::Mul => {
        
        let t1 = r.near();
        let t2 = r.near();
        let t = r.eat();
        writeln!(w, "  mul, {} ,{} ,{}", t, t1, t2)?;
      }
    }
  }
  Ok(())
}
