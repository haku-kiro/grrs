fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no file path given");

    println!("pattern: {:?}, path: {:?}", pattern, path);
}
