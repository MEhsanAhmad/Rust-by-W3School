use std::io;

fn main() {
    println!("Hello, World!");
    println!("Press Enter to exit...");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
