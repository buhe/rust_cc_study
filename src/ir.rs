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
  Add,
  Sub,
  Mul,
  Div,
  Mod,
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
  // match e {
  //   Expr::Additive(x)=> additive(stmts, x),
  // }
  bin_op(stmts, e)
}

// fn additive(stmts: &mut Vec<IrStmt>,a: &Additive) {
//   match a {
//     Additive::Add(m,a1) => {
//       multiplicative(stmts, m);
      
//       additive(stmts, a1);
//       stmts.push(IrStmt::Add);
//     },
//     Additive::Sub(m,a1)=> {
//       multiplicative(stmts, m);
      
//       additive(stmts, a1);
//       stmts.push(IrStmt::Sub);
//     },
//     Additive::Multiplicative(m) => multiplicative(stmts, m),
//   }
// }

fn bin_op(stmts: &mut Vec<IrStmt>,m: &Expr) {
  match m {
    Expr::Mul(u, m1) => {
      // unary(stmts, u);
      
      bin_op(stmts, u);
      bin_op(stmts, m1);
      stmts.push(IrStmt::Mul);
    },
    Expr::Div(u, m1) => {
      // unary(stmts, u);
      
      bin_op(stmts, u);
      bin_op(stmts, m1);
      stmts.push(IrStmt::Div);
    },
    Expr::Mod(u, m1) => {
      // unary(stmts, u);
      
      bin_op(stmts, u);
      bin_op(stmts, m1);
      stmts.push(IrStmt::Mod);
    },
    Expr::Add(m,a1) => {
      bin_op(stmts, m);
      bin_op(stmts, a1);
      
      // additive(stmts, a1);
      stmts.push(IrStmt::Add);
    },
    Expr::Sub(m,a1)=> {
      bin_op(stmts, m);
      bin_op(stmts, a1);
      
      // additive(stmts, a1);
      stmts.push(IrStmt::Sub);
    },
    Expr::Unary(u) => unary(stmts, u),
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
          // stmts.append(other)
        }
    }
}
