//! for loop:
//!
//! ```
//! (1) string:
//!         a) for c in s.chars() {...}
//!         b) for (_, c) in s.chars().enumerate() {...}
//! (2) array:
//!         for v in nums {...}
//! ```



pub fn run() {
    string_iter();
    string_enum();
    array_iter();
}

fn string_iter() {
    let s = String::from("hello");
    for c in s.chars() {
        print!("{}", c);
    }
    println!();
}

fn string_enum() {
    let s = "hello world";
    for (_, c) in s.chars().enumerate() {
        print!("{}", c);
    }
    println!();
}


fn array_iter() {
    let nums: [i32; 5] = [10, 20, 30, 40, 50];

    print!("[ ");
    for v in nums {
        print!("{} ", v);
    }
    println!("]");
}

fn vector_iter() {
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];

    print!("[ ");
    for v in nums.iter() {
        print!("{} ", v);
    }
    println!("]");
}