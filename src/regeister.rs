pub struct Regeister {
    stack: Vec<String>,
    near_stack: Vec<String>,
}

impl Regeister {
    pub fn init() -> Self {
        Self {
            stack: vec!["s3".to_string(),"s2".to_string(),"s1".to_string(),"t6".to_string(),"t5".to_string(),"t4".to_string(),"t3".to_string(),"t2".to_string(),"t1".to_string()],
            near_stack: vec![]
        }
    }
    pub fn eat(&mut self) -> String {
        let a = self.stack.pop().unwrap();
        let b = a.clone();
        self.near_stack.push(a);
        b
    }

    pub fn put(&mut self, str: String) {
        // when not live ,put stack
        self.stack.push(str);
    }

    pub fn near(&mut self) -> String{
        self.near_stack.pop().unwrap()
    }

    pub fn put_near(& mut self, str: String) {
        self.near_stack.push(str);
    }    

}

pub struct VirtualRegeister {
    stack: Vec<String>,
    near_stack: Vec<String>,
    counter: u32,
}

impl VirtualRegeister {
    pub fn init() -> Self {
        Self {
            stack: vec![],
            near_stack: vec![],
            counter: 0,
        }
    }
    pub fn eat(&mut self) -> String {
        self.stack.push(format!("T{}", self.counter));
        self.counter += 1;
        let a = self.stack.pop().unwrap();
        let b = a.clone();
        self.near_stack.push(a);
        b
    }

    pub fn near(&mut self) -> String{
        self.near_stack.pop().unwrap()
    }

    pub fn put_near(& mut self, str: String) {
        self.near_stack.push(str);
    }    

}