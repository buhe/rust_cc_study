use crate::{ir::{IrStmt, IrProg, IrFunc}, regeister::Regeister};

pub fn dataflow(p: &IrProg) -> IrProg {
    let mut r = Regeister::init();
    let mut stmts: Vec<IrStmt> = Vec::new();
    for s in p.func.stmts.iter() {
         match s {
            IrStmt::Add(_, _, _) => todo!(),
            IrStmt::Sub(_, _, _) => todo!(),
            IrStmt::Mul(_, _, _) => todo!(),
            IrStmt::Div(_, _, _) => todo!(),
            IrStmt::Mod(_, _, _) => todo!(),
            IrStmt::Neg(_, _) => todo!(),
            IrStmt::Or(_, _, _) => todo!(),
            IrStmt::And(_, _, _) => todo!(),
            IrStmt::Equal(_, _, _, _, _) => todo!(),
            IrStmt::NotEqual(_, _, _, _) => todo!(),
            IrStmt::Lt(_, _, _) => todo!(),
            IrStmt::Let(_, _, _, _, _, _, _) => todo!(),
            IrStmt::Gt(_, _, _) => todo!(),
            IrStmt::Get(_, _, _, _, _, _) => todo!(),
            IrStmt::Ldc(n, _reg) => {
                stmts.push(IrStmt::Ldc(*n, r.eat()));
            },
            IrStmt::Ret(_) => {
                stmts.push(IrStmt::Ret(r.near()));
            },
            IrStmt::Assign(_, _, _) => todo!(),
            IrStmt::Ref(_, _) => todo!(),
            IrStmt::Beq(_) => todo!(),
            IrStmt::Jmp => todo!(),
            IrStmt::Label(_) => todo!(),
        }
    }
   
    IrProg {
        func: IrFunc { name: p.func.name.clone(), stmts }
    }
}

pub struct BasicBlock {
   pub stmts: Vec<IrStmt>,
   pub edge: Option<Box<BasicBlock>>,  
}
