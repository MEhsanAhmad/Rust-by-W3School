use std::io::{self, Write}; // Added Write trait for flushing output
use std::thread;
use std::time::Duration;

fn main() {
    println!("Type something and press enter:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let clean_input = input.trim();

    println!("----------------------------------");
    println!("You entered: {}", clean_input);
    println!("Raw representation: {:?}", input);
    println!("Character count: {} letters", clean_input.len());

    // Type checking section
    if let Ok(val) = clean_input.parse::<i32>() {
        println!("Detected Type: INTEGER ({})", val);
    } else if let Ok(val) = clean_input.parse::<f64>() {
        println!("Detected Type: FLOAT ({})", val);
    } else if let Ok(val) = clean_input.parse::<bool>() {
        println!("Detected Type: BOOLEAN ({})", val);
    } else if clean_input.chars().count() == 1 {
        println!("Detected Type: CHARACTER ('{}')", clean_input);
    } else {
        println!("Detected Type: STRING (\"{}\")", clean_input);
    }
    println!("----------------------------------");

    // Ask user for exit delay in seconds
    println!("\nEnter exit delay in seconds (e.g. 5 or 10):");
    
    let mut delay_input = String::new();
    io::stdin()
        .read_line(&mut delay_input)
        .expect("Failed to read line");

    // Default to 3 seconds if non-number is entered
    let seconds: u64 = delay_input.trim().parse().unwrap_or(3);

    println!("\nClosing program...");

    // Countdown Loop
    for remaining in (1..=seconds).rev() {
        // \r resets the cursor to the beginning of the line
        print!("\rTime remaining: {}s ", remaining);
        
        // Force Rust to display text immediately without waiting for a newline
        io::stdout().flush().unwrap();
        
        thread::sleep(Duration::from_secs(1));
    }

    println!("\rDone! Goodbye.         ");
}