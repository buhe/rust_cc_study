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
  Neg,
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
    Expr::Unary(x)=> unary(stmts, x),
  }
}

fn unary(stmts: &mut Vec<IrStmt>, u: &Unary) {
  match u {
        Unary::Int(y) => stmts.insert(0,IrStmt::Ldc(*y)),
        Unary::Neg(y) => { 
          stmts.insert(0,IrStmt::Neg);
          unary(stmts, &*y)
        },
    }
}
