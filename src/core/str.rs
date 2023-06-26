// &str 是String 的切片类型。

pub fn run() {
    let s: &str = "hello";
    println!("{}", s.len());              // len  : 5
    println!("{:?}", s.as_bytes());       // bytes: [104, 101, 108, 108, 111]
    println!("{:p}", s.as_ptr());         // ptr  : 0x1026b6760
    println!("{:p}", &s);                 // &s   : 0x16dce2f08
    println!("{}", s.is_empty());         // false
    println!("{}", s.clone());            // hello
    println!("{}", s.repeat(2));       // hellohello
    println!("{}", s.to_lowercase());     // hellohello
    println!("{:?}", s.to_uppercase());   // "HELLO"
    println!("{}", hi("John Smith"));

    let first_name: &str = "Pascal";
    let last_name: String = "John Smith".to_string();

    greet(first_name);
    greet(&last_name); // `last_name` is passed by reference


}


fn hi(name: &str) -> String {
    return "hi ".to_string() + name;
}


fn greet(name: &str) {
    println!("Hello, {}!", name);
}
