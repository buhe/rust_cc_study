pub struct Regeister {
    stack: Vec<&'static str>,
    pub last: &'static str,
}

impl Regeister {
    pub fn init() -> Self {
        Self {
            stack: vec!["t2","t1","t0"],
            last: "t0"
        }
    }
    pub fn eat<'a,'b>(&'a mut self) -> &'b str {
        let a = self.stack.pop().unwrap();
        self.last = a;
        a
    }

    pub fn take<'a,'b>(&'a mut self) -> &'b str {
        self.stack[self.stack.len() - 1]
    }
}