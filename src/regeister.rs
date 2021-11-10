pub struct Regeister {
    stack: Vec<&'static str>
}

impl Regeister {
    pub fn init() -> Self {
        Self {
            stack: vec!["t0","t1","t2"]
        }
    }
    pub fn select<'a,'b>(&'a mut self) -> &'b str {
        self.stack.pop().unwrap()
    }

    pub fn select2<'a,'b>(&'a mut self) -> &'b str {
        self.stack[self.stack.len() - 1]
    }
}