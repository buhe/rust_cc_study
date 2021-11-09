use crate::ast::*;

#[derive(Debug, Clone)]
pub struct IrProg {
  pub func: IrFunc,
}

#[derive(Debug, Clone)]
pub struct IrFunc {
  pub name: String,
  pub stmts: Vec<IrStmt>,
}

#[derive(Debug, Clone)]
pub enum IrStmt {
  Ldc(i32),
  Ret,
}

pub fn ast2ir(p: &Prog) -> IrProg {
  IrProg {
    func: func(&p.func),
  }
}

fn func(f: &Func) -> IrFunc {
  let mut stmts = Vec::new();
  match &f.stmt {
    Stmt::Ret(e) => {
      expr(&mut stmts, e);
      stmts.push(IrStmt::Ret);
    }
  }
  IrFunc {
    name: f.name.clone(),
    stmts,
  }
}

fn expr(stmts: &mut Vec<IrStmt>, e: &Expr) {
  match e {
    Expr::Int(x) => stmts.push(IrStmt::Ldc(*x)),
  }
}