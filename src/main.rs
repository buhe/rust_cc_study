
fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    lang_study::run("int main() {
    return --2021;
}".to_string(), &mut std::io::stdout())
}
