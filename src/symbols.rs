use std::{collections::HashMap};

pub struct SymTab {
    table: HashMap<String, Symbol>
}
impl SymTab {
    pub fn init() -> Self {
        Self {
            table: HashMap::new()
        }
    }

    pub fn put(&mut self, name: String, sym: Symbol) {
        self.table.insert(name, sym);
    }

    pub fn extis(&mut self, name: &String) -> bool {
        self.table.contains_key(name)
    }

    pub fn get(&mut self, name: &String) -> &Symbol{
        let s = self.table.get(name);
        s.unwrap()
    }
}

pub struct Symbol {
    pub name: String,
    pub reg: Option<String>,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Self{
            name,reg: None
        }
    }
}