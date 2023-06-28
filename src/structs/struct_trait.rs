//! # trait
//!
//! ## example-1
//!
//! ```rust
//! use crate::structs::struct_trait::AnyExt;
//! mod structs;
//!
//! fn main() {
//!     println!("{}", AnyExt::type_name(&12));  // i32
//! }
//! ```




pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}


impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}


pub fn run() {
    let num: f64 = 32.90;
    println!("{}", num.type_name());   // f64
    println!("{}", num.abs());
}






