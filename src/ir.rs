use std::collections::VecDeque;

use crate::{ast::*, symbols::SymTab, regeister::VirtualRegeister};


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
  Add(String, String, String),
  Sub(String, String, String),
  Mul(String, String, String),
  Div(String, String, String),
  Mod(String, String, String),
  Neg(String, String),
  Or(String, String, String),
  And(String, String, String),
  Equal(String, String, String, String, String),
  NotEqual(String, String, String, String),
  Lt(String, String, String),
  Let(String, String, String, String, String, String, String),
  Gt(String, String, String),
  Get(String, String, String, String, String, String),
  Ldc(i32, String),
  Ret(String),
  // env, id, 2 reg
  Assign(Vec<u32>, String, String),
  // env, id
  Ref(Vec<u32>, String),
  Beq(String),
  Jmp,
  Label(String),
}

pub fn ast2ir(p: &Prog, s: &mut SymTab) -> (IrProg, BranchLabel) {
  let mut bl = BranchLabel::init();
  let mut r = VirtualRegeister::init();
  (IrProg {
    func: func(&p.func, s, &mut bl, &mut r),
  },bl)
}

fn func(f: &Func, table: &mut SymTab, bl: &mut BranchLabel,r: &mut VirtualRegeister) -> IrFunc {
  let mut stmts = Vec::new();
  block(&mut stmts, &f.stmt, table, bl, r);
  IrFunc {
    name: f.name.clone(),
    stmts,
  }
}

fn block(stmts: &mut Vec<IrStmt>,bts: &Vec<BlockItem>, table: &mut SymTab, bl: &mut BranchLabel,r: &mut VirtualRegeister) {
  for s in bts.iter() {
    match s {
        BlockItem::Stmt(s) => {
          stmt(stmts,s,table, bl, r);
        },
        BlockItem::Decl(d) => {
          if let Some(ex) = &d.expr { //when assign
            let name = &d.name;
            let scope = &d.scope;
            expr(stmts, ex, table, bl, r);
            let t2 = r.near();// todo, noy use near api
             // save
            let entry = table.entry(scope, name);
            entry.and_modify(|s| {
              if s.alloc_virtual_reg == false {
                let t = r.eat();
                s.reg = Some(t.to_string());
                s.alloc_virtual_reg = true; 
              } 
            });
            
            stmts.push(IrStmt::Assign(scope.to_vec(), name.to_string(),t2));
          }
        },
    }
  }
}

fn stmt(stmts: &mut Vec<IrStmt>,s: &Stmt, table: &mut SymTab,bl: &mut BranchLabel,r: &mut VirtualRegeister) {
  match s {
      Stmt::Ret(e) => {
        expr(stmts, e, table, bl, r);
        let t = r.near();
        stmts.push(IrStmt::Ret(t));
      }
      Stmt::Expr(e) => {
        if let Some(ex) = e {
          expr(stmts, ex, table, bl, r);
        }
      },
      Stmt::If(e, t, l) => {
        // 1. create label
        // 2. add beq ir
        // 3. when has else, add jmp ir 
        expr(stmts, e, table, bl, r);
        let reg = r.near();
        stmts.push(IrStmt::Beq(reg));
        stmt(stmts, t, table, bl, r);
        if l.is_some() {
          let s1 = l.as_ref().unwrap();
          stmts.push(IrStmt::Jmp);
          stmts.push(IrStmt::Label(bl.get(LabelType::Else)));
          stmt(stmts, s1, table, bl, r)
        } 
        stmts.push(IrStmt::Label(bl.get(LabelType::Other)));
      },
      Stmt::Block(bts) => {
        block(stmts, bts, table, bl, r)
      },
  }
}

fn expr(stmts: &mut Vec<IrStmt>, e: &Expr, table: &mut SymTab, bl: &mut BranchLabel,r: &mut VirtualRegeister) {
  bin_op(stmts, e, table, bl, r)
}

fn bin_op(stmts: &mut Vec<IrStmt>,m: &Expr, table: &mut SymTab, bl: &mut BranchLabel,r: &mut VirtualRegeister) {
  match m {
    Expr::Mul(u, m1) => {
      bin_op(stmts, u, table, bl, r);
      bin_op(stmts, m1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Mul(t1, t2, t));
    },
    Expr::Div(u, m1) => {
      bin_op(stmts, u, table, bl, r);
      bin_op(stmts, m1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Div(t1, t2, t));
    },
    Expr::Mod(u, m1) => {
      bin_op(stmts, u, table, bl, r);
      bin_op(stmts, m1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Mod(t1, t2, t));
    },
    Expr::Add(m,a1) => {
      bin_op(stmts, m, table, bl, r);
      bin_op(stmts, a1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Add(t1, t2, t));
    },
    Expr::Sub(m,a1)=> {
      bin_op(stmts, m, table, bl, r);
      bin_op(stmts, a1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Sub(t1, t2, t));
    },
    Expr::Unary(u) => unary(stmts, u, table, bl, r),
    Expr::Lt(e, e1) => {
      bin_op(stmts, e, table, bl, r);
      bin_op(stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Lt(t1, t2, t));
    }
    Expr::Gt(e, e1) => {
      bin_op(stmts, e, table, bl, r);
      bin_op(stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Gt(t1, t2, t));
    }
    Expr::Let(e, e1) => {
      bin_op(stmts, e, table, bl, r);
      bin_op(stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();

      let t3 = r.eat();
      let t6 = r.eat();

      let t4 = r.eat();

      let t5 = r.eat();
      stmts.push(IrStmt::Let(t1, t2, t, t3, t4, t5, t6));
    }
    Expr::Get(e, e1) => {
      bin_op(stmts, e, table, bl, r);
      bin_op(stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();

      let t3 = r.eat();

      let t4 = r.eat();

      let t5 = r.eat();
      stmts.push(IrStmt::Get(t1, t2, t, t3, t4, t5));
    }
    Expr::And(e, e1) => {
      bin_op(stmts, e, table, bl, r);
      bin_op(stmts, e1, table, bl, r);
      let t1 = r.near(); // s2
      let t2 = r.near(); // s1
      let t = r.eat(); // d
      stmts.push(IrStmt::And(t1, t2, t));
    }
    Expr::Or(e, e1) => {
      bin_op(stmts, e, table, bl, r);
      bin_op(stmts, e1, table, bl, r);
          let t1 = r.near();
          let t2 = r.near();
          let t = r.eat();
      stmts.push(IrStmt::Or(t1, t2, t));
    }
    Expr::NotEquals(e, e1) => {
      bin_op(stmts, e, table, bl, r);
      bin_op(stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      let t3 = r.eat();
      stmts.push(IrStmt::NotEqual(t1, t2, t, t3));
    }
    Expr::Equals(e, e1) => {
      bin_op(stmts, e, table, bl, r);
      bin_op(stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();

      let t3 = r.eat();
      let t4 = r.eat();
      stmts.push(IrStmt::Equal(t1, t2, t, t3, t4));
    }
    Expr::Assign(env,id, e) => {
      let name = &**id;
      let n = &**env;
      bin_op(stmts, e, table, bl, r);
      let t2 = r.near();// todo, noy use near api
        let entry = table.entry(n, name);
      entry.and_modify(|s| {
        if s.alloc_virtual_reg == false {
          let t = r.eat();
          s.reg = Some(t.to_string());
          s.alloc_virtual_reg = true; 
        } 
      });
      stmts.push(IrStmt::Assign(n.to_vec(), name.to_string(),t2));
    },
    Expr::Null => {},
    Expr::Cond(condition, then, other) => {
      // like if-else
      expr(stmts, condition, table, bl, r);
      let reg = r.near();
      stmts.push(IrStmt::Beq(reg));
      expr(stmts, then, table, bl, r);
      stmts.push(IrStmt::Jmp);
      stmts.push(IrStmt::Label(bl.get(LabelType::Else)));
      expr(stmts, other, table, bl, r);
      stmts.push(IrStmt::Label(bl.get(LabelType::Other)));
    },
  }
}

fn unary(stmts: &mut Vec<IrStmt>, u: &Unary, table: &mut SymTab, bl: &mut BranchLabel,r: &mut VirtualRegeister) {
  match u {
        Unary::Int(y) => {
          let t = r.eat();
          stmts.push(IrStmt::Ldc(*y, t))
        },
        Unary::Neg(y) => { 
          unary(stmts, &*y, table, bl, r);
          let t1 = r.near();
          let t2 = r.eat();
          stmts.push(IrStmt::Neg(t1, t2));
        },
        Unary::Primary(y) => {
          expr(stmts, &*y, table, bl, r)
        }
        Unary::Identifier(env, id) => {
          // check decl, table exist
          let name = &**id;
          // use var
          assert!(table.extis(env, name).0);
          println!("t:{:?}", table);
          let reg = table.get(env, name).reg.as_ref().unwrap();
          r.put_near(reg.clone());

          stmts.push(IrStmt::Ref(env.to_vec(), name.to_string()));
        },
    }
}
