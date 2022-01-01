use std::collections::VecDeque;

use crate::{ast::*, symbols::SymTab};


pub enum LabelType {
  Other,
  Else,
}
pub struct Label {
  pub name: String,
  pub lt: LabelType,
}

impl Label {
  fn new(name: String,lt: LabelType) -> Self {
    Self {name, lt}
  }
}

pub struct BranchLabel {
  counter: u32,
  queue: VecDeque<Label>,
}

impl BranchLabel {
  fn init() -> Self {
    Self {
      counter: 0,
      queue: VecDeque::new(),
    }
  }

  fn get(&mut self, lt: LabelType) -> String {
    self.counter += 1;
    let name  = format!("L{}",self.counter);
    let n = name.clone();
    self.queue.push_front(Label::new(name, lt));
    n
  }

  pub fn label(&mut self) -> Label {
    self.queue.pop_back().unwrap()
  }

  // fn reset(&mut self) {
  //   self.counter = 0;
  // }
}

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
  // env, id
  Assign(Vec<u32>, String),
  Ref(Vec<u32>, String),
  Beq,
  Jmp,
  Label(String),
}

pub fn ast2ir(p: &Prog, s: &mut SymTab) -> (IrProg, BranchLabel) {
  let mut bl = BranchLabel::init();
  (IrProg {
    func: func(&p.func, s, &mut bl),
  },bl)
}

fn func(f: &Func, table: &mut SymTab, bl: &mut BranchLabel) -> IrFunc {
  let mut stmts = Vec::new();
  block(&mut stmts, &f.stmt, table, bl);
  IrFunc {
    name: f.name.clone(),
    stmts,
  }
}

fn block(stmts: &mut Vec<IrStmt>,bts: &Vec<BlockItem>, table: &mut SymTab, bl: &mut BranchLabel) {
  for s in bts.iter() {
    match s {
        BlockItem::Stmt(s) => {
          stmt(stmts,s,table, bl);
        },
        BlockItem::Decl(d) => {
          if let Some(ex) = &d.expr {
            expr(stmts, ex, table, bl);
            let name = &d.name;
            let scope = &d.scope;
            stmts.push(IrStmt::Assign(scope.to_vec(), name.to_string()));
          }
        },
    }
  }
}

fn stmt(stmts: &mut Vec<IrStmt>,s: &Stmt, table: &mut SymTab,bl: &mut BranchLabel) {
  match s {
      Stmt::Ret(e) => {
        expr(stmts, e, table, bl);
        stmts.push(IrStmt::Ret);
      }
      Stmt::Expr(e) => {
        if let Some(ex) = e {
          expr(stmts, ex, table, bl);
        }
      },
      Stmt::If(e, t, l) => {
        // 1. create label
        // 2. add beq ir
        // 3. when has else, add jmp ir 
        expr(stmts, e, table, bl);
        stmts.push(IrStmt::Beq);
        stmt(stmts, t, table, bl);
        if l.is_some() {
          let s1 = l.as_ref().unwrap();
          stmts.push(IrStmt::Jmp);
          stmts.push(IrStmt::Label(bl.get(LabelType::Else)));
          stmt(stmts, s1, table, bl)
        } 
        stmts.push(IrStmt::Label(bl.get(LabelType::Other)));
      },
      Stmt::Block(bts) => {
        block(stmts, bts, table, bl)
      },
  }
}

fn expr(stmts: &mut Vec<IrStmt>, e: &Expr, table: &mut SymTab, bl: &mut BranchLabel) {
  bin_op(stmts, e, table, bl)
}

fn bin_op(stmts: &mut Vec<IrStmt>,m: &Expr, table: &mut SymTab, bl: &mut BranchLabel) {
  match m {
    Expr::Mul(u, m1) => {
      bin_op(stmts, u, table, bl);
      bin_op(stmts, m1, table, bl);
      stmts.push(IrStmt::Mul);
    },
    Expr::Div(u, m1) => {
      bin_op(stmts, u, table, bl);
      bin_op(stmts, m1, table, bl);
      stmts.push(IrStmt::Div);
    },
    Expr::Mod(u, m1) => {
      bin_op(stmts, u, table, bl);
      bin_op(stmts, m1, table, bl);
      stmts.push(IrStmt::Mod);
    },
    Expr::Add(m,a1) => {
      bin_op(stmts, m, table, bl);
      bin_op(stmts, a1, table, bl);
      stmts.push(IrStmt::Add);
    },
    Expr::Sub(m,a1)=> {
      bin_op(stmts, m, table, bl);
      bin_op(stmts, a1, table, bl);
      stmts.push(IrStmt::Sub);
    },
    Expr::Unary(u) => unary(stmts, u, table, bl),
    Expr::Lt(e, e1) => {
      bin_op(stmts, e, table, bl);
      bin_op(stmts, e1, table, bl);
      stmts.push(IrStmt::Lt);
    }
    Expr::Gt(e, e1) => {
      bin_op(stmts, e, table, bl);
      bin_op(stmts, e1, table, bl);
      stmts.push(IrStmt::Gt);
    }
    Expr::Let(e, e1) => {
      bin_op(stmts, e, table, bl);
      bin_op(stmts, e1, table, bl);
      stmts.push(IrStmt::Let);
    }
    Expr::Get(e, e1) => {
      bin_op(stmts, e, table, bl);
      bin_op(stmts, e1, table, bl);
      stmts.push(IrStmt::Get);
    }
    Expr::And(e, e1) => {
      bin_op(stmts, e, table, bl);
      bin_op(stmts, e1, table, bl);
      stmts.push(IrStmt::And);
    }
    Expr::Or(e, e1) => {
      bin_op(stmts, e, table, bl);
      bin_op(stmts, e1, table, bl);
      stmts.push(IrStmt::Or);
    }
    Expr::NotEquals(e, e1) => {
      bin_op(stmts, e, table, bl);
      bin_op(stmts, e1, table, bl);
      stmts.push(IrStmt::NotEqual);
    }
    Expr::Equals(e, e1) => {
      bin_op(stmts, e, table, bl);
      bin_op(stmts, e1, table, bl);
      stmts.push(IrStmt::Equal);
    }
    Expr::Assign(env,id, e) => {
      let name = &**id;
      let n = &**env;
      bin_op(stmts, e, table, bl);
      stmts.push(IrStmt::Assign(n.to_vec(), name.to_string()));
    },
    Expr::Null => {},
    Expr::Cond(condition, then, other) => {
      // like if-else
      expr(stmts, condition, table, bl);
      stmts.push(IrStmt::Beq);
      expr(stmts, then, table, bl);
      stmts.push(IrStmt::Jmp);
      stmts.push(IrStmt::Label(bl.get(LabelType::Else)));
      expr(stmts, other, table, bl);
      stmts.push(IrStmt::Label(bl.get(LabelType::Other)));
    },
  }
}

fn unary(stmts: &mut Vec<IrStmt>, u: &Unary, table: &mut SymTab, bl: &mut BranchLabel) {
  match u {
        Unary::Int(y) => stmts.push(IrStmt::Ldc(*y)),
        Unary::Neg(y) => { 
          unary(stmts, &*y, table, bl);
          stmts.push(IrStmt::Neg);
        },
        Unary::Primary(y) => {
          expr(stmts, &*y, table, bl)
        }
        Unary::Identifier(env, id) => {
          // check decl, table exist
          let name = &**id;
          // use var
          assert!(table.extis(env, name).0);
          stmts.push(IrStmt::Ref(env.to_vec(), name.to_string()));
        },
    }
}
