fn main() {
    let pattern = std::env::args().nth(1).expect("expect some pattern");
    let path = std::env::args().nth(2).expect("expect some path");

    println!("pattern: {:?}, path: {:?}", pattern, path);
}
