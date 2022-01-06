#[derive(Debug, Clone)]
pub struct Prog {
  pub funcs: Vec<Func>,
}

#[derive(Debug, Clone)]
pub struct Func {
  pub name: String,
  pub stmt: Vec<BlockItem>,
  pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct Param {
  pub name: String,
}

impl Param {
    pub fn new(name: String) -> Self{
      Self {
        name
      }
    }
}


#[derive(Debug, Clone)]
pub struct Call {
  pub name: String,
  pub params: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Decl {
  pub t: Type,
  pub name: String,
  pub expr: Option<Expr>,
  pub scope: Vec<u32>,
}
#[derive(Debug, Clone)]
pub enum BlockItem {
    Stmt(Stmt),
    Decl(Decl),
}
#[derive(Debug, Clone)]
pub enum Stmt {
  Ret(Expr),
  Expr(Option<Expr>),
  If(Expr, Box<Stmt>, Option<Box<Stmt>>),
  Block(Vec<BlockItem>),
  For(Option<Expr>,Option<Expr>,Option<Expr>, Box<Stmt>),
  While(Expr, Box<Stmt>),
  Continue,
  Break,
  // Decl(Decl),
}

#[derive(Debug, Clone)]
pub enum Type {
  Integer  
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
  Assign(Box<Vec<u32>>, Box<String>, Box<Expr>),
  Cond(Box<Expr>, Box<Expr>, Box<Expr>),
  Null,
}
#[derive(Debug, Clone)]
pub enum Unary {
  Int(i32),
  Neg(Box<Unary>),
  Primary(Box<Expr>),
  Identifier(Box<Vec<u32>>, Box<String>),
  Call(Call),
}
