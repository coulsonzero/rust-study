pub fn run() {
    hello();
    hi("John Smith");
    hey("John Smith".to_string());
    println!("{}", max(3,5));
    println!("{:?}", info("John Smith", 21))
}



fn hello() {
    println!("hello world!");
}

/// functions-params
/// ```
/// hi("John Smith")
/// ```
fn hi(name: &str) -> String {
    return "hi ".to_string() + name;
}

// hey("John Smith".to_string())
fn hey(name: String) -> String {
    return "hey ".to_string() + &name;
}

fn max(a: i32, b: i32) -> i32 {
    return if a > b {a} else {b};
}

/// multi-return-value
/// ```
/// let (a, b) = info("John Smith", 21)
/// ```
fn info(name: &str, age: i32) -> (String, i32) {
    return ("my name is ".to_string() + name, age);
}
