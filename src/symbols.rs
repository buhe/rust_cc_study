use std::{collections::{HashMap, hash_map::Entry}};

#[derive(Debug)]
pub struct SymTab {
    table: HashMap<Vec<u32>, HashMap<String, Symbol>>,
    scope_counter: HashMap<u32, u32>,
    scope_level: u32,
    pub current_scope: Vec<u32>,
}
impl SymTab {
    pub fn init() -> Self {
        Self {
            table: HashMap::new(),
            scope_counter: HashMap::new(),
            scope_level: 1,
            current_scope: vec![1],
        }
    }

    pub fn enter_scope(&mut self) {
        // change current scope
        self.scope_level += 1;
        if self.scope_counter.contains_key(&self.scope_level) {
            self.scope_counter.entry(self.scope_level).and_modify(|v| {*v +=1});
        } else {
            self.scope_counter.insert(self.scope_level, 1);
        }
        
        self.current_scope.push(*self.scope_counter.get(&self.scope_level).unwrap());
        self.table.insert(self.current_scope.to_vec(), HashMap::new());

    }

    pub fn leave_scope(&mut self) {
        // change current scope
        self.scope_level -= 1;
        self.current_scope.pop();
    }

    pub fn put(&mut self, name: String, sym: Symbol) {
        self.table.get_mut(&self.current_scope).unwrap().insert(name, sym);
    }


    pub fn extis(&self, s: &Vec<u32>, name: &String) -> (bool, Vec<u32>) {
        let mut ss = s.clone();
        while !ss.is_empty() {
            if self.table.get(&ss).unwrap().contains_key(name) {
                return  (true,ss);
            }
            ss.pop();
        }
        (false, vec![0])
        
        // get all parent scope
        // s.pop
        // all scope is not found return false, vec![-1]
    }

    pub fn entry(&mut self, s: &Vec<u32>, name: &String) -> Entry<String, Symbol>{
      self.table.get_mut(s).unwrap().entry(name.to_string())
    }

    pub fn get(&mut self, s: &Vec<u32>, name: &String) -> &Symbol{
      self.table.get(s).unwrap().get(name).unwrap()
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub reg: Option<String>,
    pub alloc_virtual_reg: bool,
    pub alloc_phy_reg: bool,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Self{
            name,
            reg: None,
            alloc_virtual_reg: false,
            alloc_phy_reg: false,
        }
    }
}

mod tests {
    use crate::symbols::Symbol;

    use super::SymTab;

    #[test]
    fn test_enter() {
        let mut s = SymTab::init();
        s.enter_scope();
        println!("{:?}", s.current_scope);
        println!("{:?}", s.table);
        s.leave_scope();
        println!("{:?}", s.current_scope);
        println!("{:?}", s.table);
    }

    #[test]
    fn test_enter2() {
        let mut s = SymTab::init();
        s.enter_scope();
        println!("{:?}", s.current_scope);
        println!("{:?}", s.table);
        s.put("x".to_string(), Symbol::new("x".to_string()));
        s.leave_scope();
        println!("{:?}", s.current_scope);
        println!("{:?}", s.table);
    }

    #[test]
    fn test_enter3() {
        let mut s = SymTab::init();
        s.enter_scope();
        println!("{:?}", s.current_scope);
        println!("{:?}", s.table);
        s.put("x".to_string(), Symbol::new("x".to_string()));
        s.enter_scope();
        println!("current scope is {:?}", s.current_scope);
        let r = s.extis(&s.current_scope, &"x".to_string());
        println!("x in {:?}", r);
        s.leave_scope();
        s.leave_scope();
        println!("{:?}", s.current_scope);
        println!("{:?}", s.table);
    }
}