//! if statement:
//!
//! ```rust
//! if condition {
//!     // statement(s)
//! } else if condition {
//!     // statement(s)
//! } else if condition {
//!     // statement(s)
//! } else {
//!     // statement(s)
//! }
//! ```
//!
//! a > b ? a : b
//! ```
//! let res = if a > b {a} else {b};
//! ```



pub fn run() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// output: number is divisible by 3