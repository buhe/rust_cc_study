
fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    lang_study::run("int main() {
    return --2021 + 33 / 11;
}".to_string(), &mut std::io::stdout())
}

mod tests {
    #[test]
    fn integer() {
        lang_study::run("int main() {
    return 2021;
}".to_string(), &mut std::io::stdout()).unwrap();
    }
    #[test]
    fn neg_integer() {
        lang_study::run("int main() {
    return --2021;
}".to_string(), &mut std::io::stdout()).unwrap();
    }
}
