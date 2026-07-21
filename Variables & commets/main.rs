use std::io;

fn main() {
    // now create one file in which one variable change 03 times;

    let mut a = String::new();

    println!("Enter Amount of a:");
    io::stdin()
        .read_line(&mut a)
        .expect("Enter correct amount");
    println!("A is Equal to = {}", a.trim());

    // 2nd line
    a.clear();
    println!("Enter Amount of a:");
    io::stdin()
        .read_line(&mut a)
        .expect("Enter correct amount");
    println!("A is Equal to = {}", a.trim());

    //3rd line
    a.clear();
    println!("Enter Amount of a:");
    io::stdin()
        .read_line(&mut a)
        .expect("Enter correct amount");
    println!("A is Equal to = {}", a.trim());


    let mut close = String::new();
    io::stdin()
        .read_line(&mut close)
        .unwrap();
}