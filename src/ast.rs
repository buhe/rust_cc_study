#[derive(Debug, Clone)]
pub struct Prog {
  pub func: Func,
}

#[derive(Debug, Clone)]
pub struct Func {
  pub name: String,
  pub stmt: Stmt,
}
#[derive(Debug, Clone)]
pub enum Stmt {
  Ret(Expr),
}
#[derive(Debug, Clone)]
pub enum Expr {
  Unary(Unary),
  Add(Box<Expr>, Box<Expr>),// left right
  Sub(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>),
  Mod(Box<Expr>, Box<Expr>),
  Lt(Box<Expr>, Box<Expr>),
  Gt(Box<Expr>, Box<Expr>),
  Let(Box<Expr>, Box<Expr>),
  Get(Box<Expr>, Box<Expr>),
  And(Box<Expr>, Box<Expr>),
  Or(Box<Expr>, Box<Expr>),
  NotEquals(Box<Expr>, Box<Expr>),
  Equals(Box<Expr>, Box<Expr>),
}
#[derive(Debug, Clone)]
pub enum Unary {
  Int(i32),
  Neg(Box<Unary>),
  Primary(Box<Expr>)
}
