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
  Assign(String),
  Ref(String),
}

pub fn ast2ir(p: &Prog, s: &mut SymTab) -> IrProg {
  IrProg {
    func: func(&p.func, s),
  }
}

fn func(f: &Func, table: &mut SymTab) -> IrFunc {
  let mut stmts = Vec::new();
  for s in (&f.stmt).iter() {
    match s {
        Stmt::Ret(e) => {
          expr(&mut stmts, e, table);
          stmts.push(IrStmt::Ret);
        }
        Stmt::Expr(e) => {
          if let Some(ex) = e {
            expr(&mut stmts, ex, table);
          }
        },
        Stmt::Decl(d) => {
          if let Some(ex) = &d.expr {
            expr(&mut stmts, ex, table);
            let name = &d.name;
            stmts.push(IrStmt::Assign(name.to_string()));
          }
        },
  }
  }
  
  IrFunc {
    name: f.name.clone(),
    stmts,
  }
}

fn expr(stmts: &mut Vec<IrStmt>, e: &Expr, table: &mut SymTab) {
  bin_op(stmts, e, table)
}

fn bin_op(stmts: &mut Vec<IrStmt>,m: &Expr, table: &mut SymTab) {
  match m {
    Expr::Mul(u, m1) => {
      bin_op(stmts, u, table);
      bin_op(stmts, m1, table);
      stmts.push(IrStmt::Mul);
    },
    Expr::Div(u, m1) => {
      bin_op(stmts, u, table);
      bin_op(stmts, m1, table);
      stmts.push(IrStmt::Div);
    },
    Expr::Mod(u, m1) => {
      bin_op(stmts, u, table);
      bin_op(stmts, m1, table);
      stmts.push(IrStmt::Mod);
    },
    Expr::Add(m,a1) => {
      bin_op(stmts, m, table);
      bin_op(stmts, a1, table);
      stmts.push(IrStmt::Add);
    },
    Expr::Sub(m,a1)=> {
      bin_op(stmts, m, table);
      bin_op(stmts, a1, table);
      stmts.push(IrStmt::Sub);
    },
    Expr::Unary(u) => unary(stmts, u, table),
    Expr::Lt(e, e1) => {
      bin_op(stmts, e, table);
      bin_op(stmts, e1, table);
      stmts.push(IrStmt::Lt);
    }
    Expr::Gt(e, e1) => {
      bin_op(stmts, e, table);
      bin_op(stmts, e1, table);
      stmts.push(IrStmt::Gt);
    }
    Expr::Let(e, e1) => {
      bin_op(stmts, e, table);
      bin_op(stmts, e1, table);
      stmts.push(IrStmt::Let);
    }
    Expr::Get(e, e1) => {
      bin_op(stmts, e, table);
      bin_op(stmts, e1, table);
      stmts.push(IrStmt::Get);
    }
    Expr::And(e, e1) => {
      bin_op(stmts, e, table);
      bin_op(stmts, e1, table);
      stmts.push(IrStmt::And);
    }
    Expr::Or(e, e1) => {
      bin_op(stmts, e, table);
      bin_op(stmts, e1, table);
      stmts.push(IrStmt::Or);
    }
    Expr::NotEquals(e, e1) => {
      bin_op(stmts, e, table);
      bin_op(stmts, e1, table);
      stmts.push(IrStmt::NotEqual);
    }
    Expr::Equals(e, e1) => {
      bin_op(stmts, e, table);
      bin_op(stmts, e1, table);
      stmts.push(IrStmt::Equal);
    }
    Expr::Assign(id, e) => {
      let name = &**id;
      // let expr = &**e;
      bin_op(stmts, e, table);
      stmts.push(IrStmt::Assign(name.to_string()));
    },
    Expr::Null => {},
  }
}

fn unary(stmts: &mut Vec<IrStmt>, u: &Unary, table: &mut SymTab) {
  match u {
        Unary::Int(y) => stmts.push(IrStmt::Ldc(*y)),
        Unary::Neg(y) => { 
          unary(stmts, &*y, table);
          stmts.push(IrStmt::Neg);
        },
        Unary::Primary(y) => {
          expr(stmts, &*y, table)
        }
        Unary::Identifier(id) => {
          // check decl, table exist
          let name = &**id;
          // use var
          assert!(table.extis(name));
          stmts.push(IrStmt::Ref(name.to_string()));
        },
    }
}
