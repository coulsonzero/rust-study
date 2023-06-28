use std::io;




pub fn example_str() {
    println!("Please enter a line: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    println!("Output: {}", s);
}

// Input : hello world!
// Output: hello world!

pub fn example_num() {
    // --snip--
    println!("Please enter a Number: ");
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    let num: u32 = s.trim().parse()
        .expect("Please type a number!");
    println!("Output: {}", num);
}