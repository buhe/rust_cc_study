pub struct Regeister {
    stack: Vec<&'static str>,
    near_stack: Vec<&'static str>,
}

impl Regeister {
    pub fn init() -> Self {
        Self {
            stack: vec!["t9","t8","t7","t6","t5","t4","t3","t2","t1","t0"],
            near_stack: vec![]
        }
    }
    pub fn eat<'a,'b>(&'a mut self) -> &'b str {
        let a = self.stack.pop().unwrap();
        self.near_stack.push(a);
        a
    }

    pub fn near<'a,'b>(&'a mut self) -> &'b str{
        self.near_stack.pop().unwrap()
    }

    // pub fn take<'a,'b>(&'a mut self) -> &'b str {
    //     self.stack[self.stack.len() - 1]
    // }
}