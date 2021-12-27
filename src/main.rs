
fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    lang_study::run("int main() {
    return --2021 + 33 / 11;
}".to_string(), &mut std::io::stdout())
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
}
