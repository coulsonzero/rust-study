/*
Primitive Types--
Integers: i8, i16, i32, i64, i128, isize; u8, u16, u32, u64, u128, usize; (number)
Floats: f32, f64  (number)
Bool: true, false (boolean)
Characters: 'a' (char)
Tuples: (i8, i16, i32, i64, i128, u8, u16, u32, u64, u128) (tuple)
Arrays: [i32; 3] (array)

Rust is a statically typed language, which means that the types of
the variables must be known at compile time, and the compiler will
not allow you to assign a value of the wrong type.

*/




use crate::structs::struct_trait::AnyExt;

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 50000;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    // char
    let a1 = 'a';
    let heart_eyed_cat = '😻';
    let face = '\u{1F600}';

    // string
    let s:  &str   = "Hello, world!";
    let s1: String = String::from("hello world");
    let s2: String = "Hello".to_string();
    let s3: String = "also this".into();


    // array (same value type)
    let nums: [i32; 5] = [1, 2, 3, 4, 5];

    // tuple (different value types, fixed size)
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face, nums));
}


#[test]
fn test_datetype_case() {
    let num: f64 = 12.7;
    // println!("{}", num as i32);         // 12
    assert_eq!(num as i32, 12);
}

#[test]
fn test_typename() {
    let num: f64 = 12.7;
    // println!("{}", num.type_name());    // f64
    // println!("{}", type_name2(num));    // f64
    assert_eq!(num.type_name(), "f64");
    assert_eq!(type_name2(num), "f64");
}

fn type_name<T>() -> &'static str {
    return std::any::type_name::<T>()
}


fn type_name2<T>(_: T) -> &'static str {
    return std::any::type_name::<T>()
}