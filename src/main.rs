use std::fs::{File, self};


fn main() -> std::io::Result<()> {
    let mut f = File::create("asm.S")?;
    lang_study::run("int main() { int x = 1; if (x) x = 2; else x = 3; return x; }".to_string(), &mut f)?;
    println!("asm:\n {}", fs::read_to_string("asm.S")?);
    Ok(())
}

mod tests {
    #[test]
    fn integer() {
        lang_study::run("int main() { return 2021; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }
    #[test]
    fn neg_integer() {
        lang_study::run("int main() { return --2021; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

     #[test]
    fn par_expr() {
        lang_study::run("int main() { return (--2021); }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn _4_expr() {
        lang_study::run("int main() { return 1 + 1; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn _4_expr2() {
        lang_study::run("int main() { return 1 + 1 * 2 * 3; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn _4_expr3() {
        lang_study::run("int main() { return 1 - 2 - 3 - 4; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn log_1() {
        lang_study::run("int main() { return 2 >= 1; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn log_2() {
        lang_study::run("int main() { return 2 && 1; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn log_3() {
        lang_study::run("int main() { return 2 > 1; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn decl_1() {
        lang_study::run("int main() { int x = 2021; return x; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn decl_2() {
        lang_study::run("int main() { int x = 2021; 1+1; return x; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn decl_3() {
        lang_study::run("int main() { int x = 2021; 1+1;1+x; return x; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn decl_4() {
        lang_study::run("int main() { int x = 2021; int y = 1+1;1+x; return y; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    #[should_panic]
    fn decl_5() {
        lang_study::run("int main() { return x; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn decl_6() {
        lang_study::run("int z() { int x = 2021; return x; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn if_1() {
        lang_study::run("int z() { int x = 1; if (x) x = 2; else x = 3; return x; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn if_2() {
        lang_study::run("int z() { if (3) 4; else 5; return 6; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn if_3() {
        lang_study::run("int z() { int x = 1 ? 2 : 3; return x; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn scope_1() {
        lang_study::run("int main() { int x = 1;   { x = 2; int x = 3; } x = 4; return x; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn for_1() {
        lang_study::run("int main() { int i; for (i = 0; i < 5; i = i + 1) {break;} }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn while_1() {
        lang_study::run("int main() { int i = 2; while(i == 5) {i = i + 1; continue;} }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }
}
