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

  // fn num(&mut self,t: &Token) -> Expr {
  //   match t.ty {
  //     TokenType::Num(val) => Expr::Unary(Unary::Int(val)),
  //     _ => self.bad_token("number expected from num"),
  //   }
  // }

  fn expr(&mut self) -> Expr {
    let t = &self.tokens[self.pos];
    self.pos += 1;
    let mut child:Vec<Unary> = vec![];
    match t.ty {
      TokenType::Neg => self.unary(&mut child),
      TokenType::Num(val) => Expr::Unary(Unary::Int(val)),
      _ => self.bad_token("number expected from expr"),
    }
  }

  fn unary(&mut self, c: &mut Vec<Unary>) -> Expr {
    Expr::Unary(Unary::Neg(Box::new(self._unary(c))))
  }

  fn _unary(&mut self, c: &mut Vec<Unary>) -> Unary {
    let t = &self.tokens[self.pos];
    self.pos += 1;
    match t.ty {
      TokenType::Neg => {
        let a = Unary::Neg(Box::new(self._unary(c)));
        c.insert(0, a.clone());
        a
      },
      TokenType::Num(val) => Unary::Int(val),
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
