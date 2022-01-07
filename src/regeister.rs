pub struct Regeister {
    stack: Vec<String>,
    near_stack: Vec<String>,
    args_stack: Vec<String>,
    args: Vec<String>,
}

impl Regeister {
    pub fn init() -> Self {
        Self {
            args: vec![],
            stack: vec!["t6".to_string(),"t5".to_string(),"t4".to_string(),"t3".to_string(),"t2".to_string(),"t1".to_string(),"t0".to_string()],
            near_stack: vec![],
            //todo! every func
            args_stack: vec!["a6".to_string(),"a5".to_string(),"a4".to_string(),"a3".to_string(),"a2".to_string(),"a1".to_string(),"a0".to_string(),],
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

    pub fn put_near(&mut self, str: String) {
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

    pub fn put_near(&mut self, str: String) {
        self.near_stack.push(str);
    }  
    
}

pub struct ArgTunnel {

}

impl ArgTunnel {
    pub fn init() -> Self {
        Self {
          
        }
    }
      // def func
    pub fn set_arg(&mut self, func_name: &String) -> String {
        let arg = self.args_stack.pop().unwrap();
        self.args.push(arg.clone());
        arg
    }

    // call func
    pub fn get_arg(&mut self, func_name: &String) -> String {
        let arg = self.args.pop().unwrap();
        self.args_stack.push(arg.clone());
        arg
    }
}