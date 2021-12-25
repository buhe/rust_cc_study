use crate::ast::*;
use crate::lexer::{Token, TokenType};
use crate::symbols::{SymTab, Symbol};

pub fn parsing(tokens: &Vec<Token>) -> (Prog, SymTab) {
  let mut parser = Parser::new(tokens.to_vec());
  parser.prog();
  if let Some(prog) = parser.prog {
    return (prog, parser.symbols);
  } else {
    panic!("Error in parsing");
  }
}

pub struct Parser {
  tokens: Vec<Token>,
  pos: usize,
  prog: Option<Prog>,
  pub symbols: SymTab,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens,
      pos: 0,
      prog: None,
      symbols: SymTab::init(),
    }
  }

  // pub fn get_table(&self) -> SymTab {
  //   self.symbols
  // }

  pub fn bad_token(&self, msg: &str) -> ! {
    panic!("{}", msg);
  }

  fn expect(&mut self, ty: TokenType) {
    let t = &self.tokens[self.pos];
    if t.ty != ty {
      self.bad_token(&format!("{:?} expected , actual {:?}", ty, t.ty));
    }
    self.pos += 1;
  }

  fn expr(&mut self) -> Expr {
    self.assignment()
  }

  fn assignment(&mut self) -> Expr {
    let t = &self.tokens[self.pos];
    if let TokenType::Ident(id) = &t.ty { // refactor to &&
      self.pos += 1; // eat id
      let t2 = &self.tokens[self.pos];
      if let TokenType::Equal = &t2.ty {
          self.pos += 1; // eat =
          Expr::Assign(Box::new(id.to_string()), Box::new(self.expr()))
      } else {
          self.pos -= 1;
          self.logical_or()
      }
    } else {
        self.logical_or()
    }
  }

  fn logical_or(&mut self) -> Expr {
    let and = self.logical_and();
    self.rest3(and) 
  }

  fn rest3(&mut self, e: Expr) -> Expr {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Or => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
            TokenType::Or => {
              self.pos += 1;
              let and = self.logical_and();
              let e1 = Expr::Or(Box::new(e), Box::new(and));
              self.rest3(e1)
            }
            _ => e
        }
      }
      _ => e
    } 
  }

  fn logical_and(&mut self) -> Expr {
    let eq = self.equality();
    self.rest4(eq)
  }

  fn rest4(&mut self, e: Expr) -> Expr {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::And => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
            TokenType::And => {
              self.pos += 1;
              let eq = self.equality();
              let e1 = Expr::And(Box::new(e), Box::new(eq));
              self.rest4(e1)
            }
            _ => e
        }
      }
      _ => e
    }
  }

  fn equality(&mut self) -> Expr {
    let r = self.relational();
    self.rest5(r)
  }

  fn rest5(&mut self, e: Expr) -> Expr {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Equal => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
          TokenType::Equal => {
            self.pos += 1;
            let r = self.relational();
            let e1 = Expr::Equals(Box::new(e), Box::new(r));
            self.rest5(e1)
          }
          _ => e
        }
      }
      TokenType::Not => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
          TokenType::Equal => {
            self.pos += 1;
            let r = self.relational();
            let e1 = Expr::NotEquals(Box::new(e), Box::new(r));
            self.rest5(e1)
          }
          _ => e
        }
       
      }
      _ => e
    }
  }
  fn relational(&mut self) -> Expr {
    let a = self.additive();
    self.rest6(a)
  }

  fn rest6(&mut self, e: Expr) -> Expr {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Lt => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
          TokenType::Equal => {
            self.pos += 1;
            let a = self.additive();
            let e1 = Expr::Let(Box::new(e), Box::new(a));
            self.rest6(e1)
          }
          _ => {
            let a = self.additive();
            let e1 = Expr::Lt(Box::new(e), Box::new(a));
            self.rest6(e1)
          }
        }
      }
      TokenType::Gt => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
          TokenType::Equal => {
            self.pos += 1;
            let a = self.additive();
            let e1 = Expr::Get(Box::new(e), Box::new(a));
            self.rest6(e1)
          }
          _ => {
            let a = self.additive();
            let e1 = Expr::Gt(Box::new(e), Box::new(a));
            self.rest6(e1)
          }
        }
      }
      _ => e
    }
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
    match &t.ty {
      TokenType::Neg => Unary::Neg(Box::new(self.unary())),
      TokenType::Num(val) => Unary::Int(*val),
      TokenType::LeftParen => {
        let r = Unary::Primary(Box::new(self.expr()));
        self.pos += 1; //skip right paren
        r
      },
      TokenType::Ident(id) => {
        // let s = self.symbols.get(&id.to_string());
        Unary::Identifier(Box::new(id.clone()))
      }
      _ => self.bad_token(&format!("expected , actual {:?}", t.ty)),
    }
  }

  fn _type(&mut self) -> Type{
      self.expect(TokenType::Int);
      Type::Integer
  }
  fn decl_id(&mut self) -> String {
    let token = &self.tokens[self.pos];
    let name;
    if let TokenType::Ident(id) = &token.ty {
      name = id
    } else {
       self.bad_token("ident expected");
    }
    self.pos += 1;
    name.to_string()
  }

  fn decl_expr(&mut self) -> Option<Expr> {
    let token = &self.tokens[self.pos];
    self.pos += 1;
    match token.ty {
        TokenType::Equal => {
          let e = self.expr(); 
          self.expect(TokenType::Semicolon);
          Some(e)
        },
        TokenType::Semicolon => None, // option expr
        _ => self.bad_token("; or expr expected from decl"),
    }
    
  }
  fn decl(&mut self) -> Decl { // decl is special assign then add assign ir.
    let t = self._type();
    let name = self.decl_id();
    let expr = self.decl_expr();
    // self.expect(TokenType::Semicolon);
    self.symbols.put(name.clone(), Symbol::new(name.clone()));  
    Decl { t, name, expr }
  }
  
  fn stmt(&mut self) -> Option<Stmt> {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Return => {
        self.pos += 1; // eat return
        let e = Stmt::Ret(self.expr()); // return must has expr
        self.expect(TokenType::Semicolon);
        Some(e)
      }
      TokenType::Int => {
        let decl = self.decl();
        Some(Stmt::Decl(decl))
      }
      TokenType::Semicolon => {
        self.pos += 1; // eat ;
        Some(Stmt::Expr(Some(Expr::Null)))
      } // branch 0 stmt
      TokenType::RightBrace => None, // when } finish.
      _ => {
          let e = self.expr(); 
          self.expect(TokenType::Semicolon);
          Some(Stmt::Expr(Some(e)))
      }
    }
  }

  fn func(&mut self) -> Func {
    self._type();
    // self.expect(TokenType::Int);
    self.expect(TokenType::Ident("main".to_string()));
    self.expect(TokenType::LeftParen);
    self.expect(TokenType::RightParen);
    self.expect(TokenType::LeftBrace);
    let mut body = vec![];
    loop { // branch mutli stmt
        let stmt = self.stmt();
        match stmt {
            Some(s) => body.push(s),
            None => break
        }
    }
    
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
