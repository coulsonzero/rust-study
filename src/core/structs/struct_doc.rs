//! # Struct
//! 
//! 1. `struct`： fields
//! ```rust
//! struct Animal {
//!     name: String,
//!     age: i32
//! }
//! ```
//! 
//! 2. `supper`： to inherit the basic structure.
//! ```rust
//! struct Animal {
//!     name: String,
//!     age: i32
//! }
//! 
//! struct Cat {
//!     supper: Animal,     /// inherit
//!     type_name: i32
//! }
//! ```
//! 3. `impl`： methods
//! ```rust
//! struct Rectangle {
//!     width: u32,
//!     height: u32,
//! }
//! 
//! 
//! impl Rectangle {
//!     fn new(w: u32, h: u32) -> Rectangle {
//!         return  Rectangle {  width: w,  height: h };
//!     }
//! 
//!     fn area(&self) -> u32 {
//!         return self.width * self.height
//!     }
//! 
//!     fn can_hold(&self, other: &Rectangle) -> bool {
//!         self.width > other.width && self.height > other.height
//!     }
//! 
//!     fn square(size: u32) -> Self {
//!         Self { width: size, height: size }
//!     }
//! }
//! 
//! #[test]
//! fn test_impl() {
//!     let rect1 = Rectangle {
//!         // code 
//!     }
//! 
//!     /// impl methods
//!     println!("area: {} ", rect1.area());
//!     let rect2 = Rectangle::new(20, 12);
//!     println!("{} {}", rect.height, rect.width);
//! }
//! ```
//! 
//! 4. `trait`: different structs use the same methods.
//!     a) trait_name::struct_name::impl_for_method();
//!     b) struct_ini.impl_for_method()
//! 
//! ```rust
//! struct Tweet {
//!     // code 
//! }
//! struct NewsArticle {
//!     // code
//! }
//! 
//! trait Summary {
//!     fn summarize(&self) -> String;
//! }
//! 
//! impl Summary for Tweet {
//!     fn summarize(&self) -> String {
//!         // code
//!     }
//! }
//! 
//! impl Summary for NewsArticle {
//!     fn summarize(&self) -> String {
//!         // code
//!     }
//! }
//! 
//! 
//! #[test]
//! fn test_trait_summary() {
//!     let tweet = Tweet {
//!         // code
//!     }
//!     let article = NewsArticle {
//!         // code
//!     }
//! 
//!     println!("{}", article.summarize());
//!     println!("{}", tweet.summarize());
//! 
//!     /// use trait method;
//!     println!("{}", Summary::summarize(&article));
//!     println!("{}", Summary::summarize(&tweet));
//! }
//! ```



