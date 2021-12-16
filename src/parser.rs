use crate::ast::*;
use crate::lexer::{Token, TokenType};

pub fn parsing(tokens: &Vec<Token>) -> Prog {
  let mut parser = Parser::new(tokens.to_vec());
  parser.prog();
  if let Some(prog) = parser.prog {
    return prog;
  } else {
    panic!("Error in parsing");
  }
}

pub struct Parser {
  tokens: Vec<Token>,
  pos: usize,
  prog: Option<Prog>,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens,
      pos: 0,
      prog: None,
    }
  }

  pub fn bad_token(&self, msg: &str) -> ! {
    panic!("{}", msg);
  }

  fn expect(&mut self, ty: TokenType) {
    let t = &self.tokens[self.pos];
    if t.ty != ty {
      self.bad_token(&format!("{:?} expected", ty));
    }
    self.pos += 1;
  }

  fn expr(&mut self) -> Expr {
    Expr::Additive(self.additive())
  }

  fn additive(&mut self) -> Additive {
    // let t = &self.tokens[self.pos];
    // self.pos += 1;
    // let a = Additive::Multiplicative(self.multiplicative());
    let m = self.multiplicative();
    self.rest(m)
    // a
  }

  fn rest(&mut self, m: Multiplicative) -> Additive{
    let t = &self.tokens[self.pos];
    // self.pos += 1;
    match t.ty {
      TokenType::Add => {
        self.pos += 1;
        let m1 = self.multiplicative();
        // self.rest();
        Additive::Add(m, Box::new(self.rest(m1)))
      }
      TokenType::Neg => {
        self.pos += 1;
        let m1 = self.multiplicative();
        // self.rest();
        Additive::Sub(m, Box::new(self.rest(m1)))
      }
      _ => {
        // self.pos -= 1;
        Additive::Multiplicative(m)
      },
    }
  }

  fn multiplicative(&mut self) -> Multiplicative {
    // let t = &self.tokens[self.pos];
    // self.pos += 1;
    // let u = Multiplicative::Unary(self.unary());
    let u = self.unary();
    self.rest2(u)
    // u
  }

  fn rest2(&mut self, u: Unary) -> Multiplicative{
    let t = &self.tokens[self.pos];
    // self.pos += 1;
    match t.ty {
      TokenType::Mod | TokenType::Mul | TokenType::Div => {
        self.pos += 1;
        let u1 = self.unary();
        Multiplicative::Div(u, Box::new(self.rest2(u1)))
      }
      _ => {
        // self.pos -= 1;
        Multiplicative::Unary(u)
      },
    }
  }

  fn unary(&mut self) -> Unary {
    let t = &self.tokens[self.pos];
    self.pos += 1;
    match t.ty {
      TokenType::Neg => Unary::Neg(Box::new(self.unary())),
      TokenType::Num(val) => Unary::Int(val),
      TokenType::LeftParen => {
        let r = Unary::Primary(Box::new(self.expr()));
        self.pos += 1; //skip right paren
        r
      },
      _ => self.bad_token("number expected from _unary"),
    }
  }

  fn stmt(&mut self) -> Stmt {
    let t = &self.tokens[self.pos];
    self.pos += 1;
    match t.ty {
      TokenType::Return => {
        let e = Stmt::Ret(self.expr());
        self.expect(TokenType::Semicolon);
        e
      }
      _ => self.bad_token("stmt error"),
    }
  }

  fn func(&mut self) -> Func {
    self.expect(TokenType::Int);
    self.expect(TokenType::Ident("main".to_string()));
    self.expect(TokenType::LeftParen);
    self.expect(TokenType::RightParen);
    self.expect(TokenType::LeftBrace);
    let body = self.stmt();
    self.expect(TokenType::RightBrace);

    Func {
      name: "main".to_string(),
      stmt: body,
    }
  }

  //fn prog(&mut self) -> Option<Prog> {

  fn prog(&mut self) {
    // Function
    self.prog = Some(Prog { func: self.func() });
    //   self.prog
  }
}
