//! # type_of (marco)
//!
//! ## example
//!
//! ```
//! pub fn type_name<T>(_: T) -> &'static str {
//!     std::any::type_name::<T>()
//! }
//!
//! #[macro_export]
//! macro_rules! type_of {
//!     () => {
//!         eprintln!("[{}, {}]", file!(), line!());
//!     };
//!     ($val:expr) => (
//!         eprintln!("[\x1b[92m{}\x1b[0m, {}], \x1b[93m${:14}\x1b[0m: {}",
//!             file!(), line!(), stringify!($val), type_name($val)
//!         );
//!     );
//! }
//!
//! println!("{}", type_name(32));            // i32
//! println!("{}", type_name(3.14));          // f64
//! println!("{}", type_name(false));         // bool
//! println!("{}", type_name('k'));           // char
//! println!("{}", type_name("hello"));       // &str
//! println!("{}", type_name([1,3, 5]));      // [i32; 3]
//! println!("{}", type_name(vec![1,3, 5]));  // alloc::vec::Vec<i32>
//!
//! type_of!();               // [src/main.rs, 27]
//! type_of!(12);             // [src/main.rs, 28] $12            : i32
//! type_of!(vec![1, 3, 5]);  // [src/main.rs, 29] $vec![1, 3, 5] : alloc::vec::Vec<i32>
//! ```


#[macro_export]
macro_rules! type_of {
    () => {
        eprintln!("[{}, {}]", file!(), line!());
    };
    ($val:expr) => (
        // 1. first output
        // eprintln!("[{}:{}] ${:14}: {}", file!(), line!(), stringify!($val), type_name($val));
        // 2. second output
        eprintln!("[\x1b[92m{}\x1b[0m, {}], \x1b[93m${:14}\x1b[0m: {}", file!(), line!(), stringify!($val), type_name($val));
        // 3. third output
        // eprintln!("[\x1b[92m{}\x1b[0m, {}]", file!(), line!());
        // eprintln!("\x1b[93m${:14}\x1b[0m: {}", stringify!($val), type_name($val));
    );
}

pub fn type_name<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}



pub fn example() {
    println!("{}", type_name(32));      // i32
    println!("{}", type_name(3.14));    // f64
    println!("{}", type_name(false));   // bool
    println!("{}", type_name('k'));     // char
    println!("{}", type_name("hello")); // &str
    println!("{}", type_name([1,3, 5])); // [i32; 3]
    println!("{}", type_name(vec![1,3, 5])); // alloc::vec::Vec<i32>

    type_of!();               // [src/main.rs, 27]
    type_of!(12);             // [src/main.rs, 28] $12            : i32
    type_of!(vec![1, 3, 5]);  // [src/main.rs, 29] $vec![1, 3, 5] : alloc::vec::Vec<i32>
}


#[deprecated]
#[cfg(test)]
#[macro_export]
macro_rules! type_of2 {
    () => {
        eprintln!("[{}:{}]", file!(), line!());
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                let (type_,tmp) = type_of2(tmp);
                // eprintln!("[{0}:{1}] {2}: {3}", file!(), line!(), stringify!($val), type_);
                eprintln!("|{:16}| {}", stringify!($val), type_);
                // eprintln!("{}", type_);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::type_of!($val)),+,)
    };
}