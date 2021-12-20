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
    self.additive()
  }

  fn additive(&mut self) -> Expr {
    let m = self.multiplicative();//1
    self.rest(m)//-2-3
  }

  fn rest(&mut self, a: Expr) -> Expr{
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Add => {
        self.pos += 1;
        let m1 = self.multiplicative(); // 2
                                                     // 3
        let a1 = Expr::Add(Box::new(a), Box::new(m1));
        self.rest(a1)
      }
      TokenType::Neg => {
        self.pos += 1;
        let m1 = self.multiplicative(); // 2
                                                     // 3
        let a1 = Expr::Sub(Box::new(a), Box::new(m1));
        self.rest(a1)
      }
      _ => {
        a
      },
    }
  }

  fn multiplicative(&mut self) -> Expr {
    let u = self.unary();
    self.rest2(Expr::Unary(u))
  }

  fn rest2(&mut self, m: Expr) -> Expr {
    let t = &self.tokens[self.pos];
    match t.ty {
       TokenType::Div => {
        self.pos += 1;
        let u1 = self.unary();
        let m1 = Expr::Div(Box::new(m), Box::new(Expr::Unary(u1)));
        self.rest2(m1)
      }
      TokenType::Mod => {
        self.pos += 1;
        let u1 = self.unary();
        let m1 = Expr::Mod(Box::new(m), Box::new(Expr::Unary(u1)));
        self.rest2(m1)
      }
      TokenType::Mul => {
        self.pos += 1;
        let u1 = self.unary();
        let m1 = Expr::Mul(Box::new(m), Box::new(Expr::Unary(u1)));
        self.rest2(m1)
      }
      _ => {
        m
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


  fn prog(&mut self) {
    // Function
    self.prog = Some(Prog { func: self.func() });
    //   self.prog
  }
}
