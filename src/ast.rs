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
  // Unary(Unary),
  Additive(Additive)
}

#[derive(Debug, Clone)]
pub enum Additive {
  Multiplicative(Multiplicative),
  Add(Multiplicative, Box<Additive>),
  Sub(Multiplicative, Box<Additive>),
}

#[derive(Debug, Clone)]
pub enum Multiplicative {
  Unary(Unary),
  Mul(Unary, Box<Multiplicative>),
  Div(Unary, Box<Multiplicative>),
  Mod(Unary, Box<Multiplicative>),
}

#[derive(Debug, Clone)]
pub enum Unary {
  Int(i32),
  Neg(Box<Unary>),
  Primary(Box<Expr>)
}
