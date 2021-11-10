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
        let t1 = r.select();
        let t2 = r.select2();
        writeln!(w, "  {} = neg {}", t1,t2)?;
        writeln!(w, "  sw t0, 0(sp)")?;
      }
      IrStmt::Ldc(x) => {
        writeln!(w, "  li t0, {}", x)?;
        writeln!(w, "  sw t0, -8(sp)")?;
        writeln!(w, "  add sp, sp, -8")?;
      }
      IrStmt::Ret => {
        writeln!(w, "  lw a0, 0(sp)")?;
        writeln!(w, "  add sp, sp, 8")?;
        writeln!(w, "  ret")?;
      }
    }
  }
  Ok(())
}
