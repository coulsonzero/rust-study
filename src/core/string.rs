

/// # String
///
/// ```rust
/// let s: &str = "Hello, world!";
/// let s: String = String::from("hello world");
/// let s: String = "Hello".to_string();
/// let s: String = "also this".into();
/// ```
///
/// ```rust
/// for c in s.chars() {
///     print!("{}", c)
/// }
/// ```
///
///
///
///




pub fn hi(name: String) -> String {
    // let s: String = String::from("hello ");
    // return s + &name;
    return "hello ".to_string() + &name;
}


pub fn string_init() {
    // initialize string
    let s:  &str   = "Hello, world!";
    let s: String = String::from("hello world");
    let s: String = "Hello".to_string();
    let s: String = "also this".into();

    let index = String::new();
}

pub fn run() {
    let s: &str = "Hello, world!";
    println!("{}", s);
    println!("{:p}", &s);
    for c in s.chars() {
        print!("{}", c)
    }
    println!("\n{}", s.len());



    let mut str: String = "hello".to_string();
    println!("\n{}", str.capacity());
    str.push(',');
    str.push_str("world");
    str.insert(2, 'c');
    // str.clear();
    str.pop();


    // s[0..1] 的类型为 str是string 的切片类型，不能直接切片，
    let s1 = &s[0..1];
    let s2 = &s[0..3];
    let s3 = &s[3..];
    
    println!("{}", str);
    
    // format
    let (name, age, country) = ("John Smith", 21, "USA");
    let s = format!("{}, by {} ({})", name, age, country);
    println!("{}", s);
}