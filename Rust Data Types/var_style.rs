use std::io;
use std::thread; // Required for sleeping/pausing
use std::time::Duration; // Required for defining time intervals


fn main() {
    println!("Type someting and press enter:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("you entered: {}", input);
    println!("you entered: {}", input.trim());

    println!("Raw representative: {:?}", input);

    println!("Character count: {} letters", input.trim().len());


    let mut close = String::new();
    io::stdin()
        .read_line(&mut close)
        .unwrap();
    
    thread::sleep(Duration::from_secs(3));
}