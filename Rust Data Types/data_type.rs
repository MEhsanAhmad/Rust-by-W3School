use std::io;

fn main() {
    without_datatype();
    // with_datatype();
    // check_datatype();

    println!("\n\nEnter any key to close!");

    let mut close = String::new();

    io::stdin()
        .read_line(&mut close)
        .expect("Failed to read line");
}

fn without_datatype() {
    let my_num = 5;         // i32
    let my_double = 5.99;   // f64
    let my_letter = 'D';    // char
    let my_bool = true;     // bool
    let my_text = "Hello";  // &str

    println!(
        "this is integer = {},\nthis is float = {},\nthis is character = {},\nthis is boolean = {},\nthis is string = {}",
        my_num, my_double, my_letter, my_bool, my_text
    ); 
}




/* ****************my name is MEA***************************** */