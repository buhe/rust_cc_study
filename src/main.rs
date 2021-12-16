
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
        lang_study::run("int main() { return 1 + 1 * 2; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn _4_expr3() {
        lang_study::run("int main() { return 1 - 2 - 3; }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }
}
