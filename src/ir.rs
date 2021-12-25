use crate::{ast::*, symbols::SymTab};

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
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Neg,
  Or,
  And,
  Equal,
  NotEqual,
  Lt,
  Let,
  Gt,
  Get,
  Ldc(i32),
  Ret,
  Assign,
}

pub fn ast2ir(p: &Prog, s: &SymTab) -> IrProg {
  IrProg {
    func: func(&p.func),
  }
}

fn func(f: &Func) -> IrFunc {
  let mut stmts = Vec::new();
  for s in (&f.stmt).iter() {
    match s {
        Stmt::Ret(e) => {
          expr(&mut stmts, e);
          stmts.push(IrStmt::Ret);
        }
        Stmt::Expr(e) => {
          if let Some(ex) = e {
            expr(&mut stmts, ex);
          }
        },
        Stmt::Decl(_d) => {

        },
  }
  }
  
  IrFunc {
    name: f.name.clone(),
    stmts,
  }
}

fn expr(stmts: &mut Vec<IrStmt>, e: &Expr) {
  bin_op(stmts, e)
}

fn bin_op(stmts: &mut Vec<IrStmt>,m: &Expr) {
  match m {
    Expr::Mul(u, m1) => {
      bin_op(stmts, u);
      bin_op(stmts, m1);
      stmts.push(IrStmt::Mul);
    },
    Expr::Div(u, m1) => {
      bin_op(stmts, u);
      bin_op(stmts, m1);
      stmts.push(IrStmt::Div);
    },
    Expr::Mod(u, m1) => {
      bin_op(stmts, u);
      bin_op(stmts, m1);
      stmts.push(IrStmt::Mod);
    },
    Expr::Add(m,a1) => {
      bin_op(stmts, m);
      bin_op(stmts, a1);
      stmts.push(IrStmt::Add);
    },
    Expr::Sub(m,a1)=> {
      bin_op(stmts, m);
      bin_op(stmts, a1);
      stmts.push(IrStmt::Sub);
    },
    Expr::Unary(u) => unary(stmts, u),
    Expr::Lt(e, e1) => {
      bin_op(stmts, e);
      bin_op(stmts, e1);
      stmts.push(IrStmt::Lt);
    }
    Expr::Gt(e, e1) => {
      bin_op(stmts, e);
      bin_op(stmts, e1);
      stmts.push(IrStmt::Gt);
    }
    Expr::Let(e, e1) => {
      bin_op(stmts, e);
      bin_op(stmts, e1);
      stmts.push(IrStmt::Let);
    }
    Expr::Get(e, e1) => {
      bin_op(stmts, e);
      bin_op(stmts, e1);
      stmts.push(IrStmt::Get);
    }
    Expr::And(e, e1) => {
      bin_op(stmts, e);
      bin_op(stmts, e1);
      stmts.push(IrStmt::And);
    }
    Expr::Or(e, e1) => {
      bin_op(stmts, e);
      bin_op(stmts, e1);
      stmts.push(IrStmt::Or);
    }
    Expr::NotEquals(e, e1) => {
      bin_op(stmts, e);
      bin_op(stmts, e1);
      stmts.push(IrStmt::NotEqual);
    }
    Expr::Equals(e, e1) => {
      bin_op(stmts, e);
      bin_op(stmts, e1);
      stmts.push(IrStmt::Equal);
    }
    Expr::Assign(id, e) => {
      let name = &**id;
      // let expr = &**e;
      bin_op(stmts, e);

    },
    Expr::Null => {},
  }
}

fn unary(stmts: &mut Vec<IrStmt>, u: &Unary) {
  match u {
        Unary::Int(y) => stmts.push(IrStmt::Ldc(*y)),
        Unary::Neg(y) => { 
          unary(stmts, &*y);
          stmts.push(IrStmt::Neg);
        },
        Unary::Primary(y) => {
          expr(stmts, &*y)
        }
        Unary::Identifier(id) => {
          // todo check

        },
    }
}
