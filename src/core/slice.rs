//! # Array
//!
//! ```rust
//! // method-1 (good: get or set)
//! for i in 0..nums.len() {
//!     // get: print!("{} ", nums[i]);
//!     // set: nums[i] += 1;
//! }
//!
//! // method-2 (only get)
//! for v in nums {
//!     print!("{} ", v);
//! }
//!
//! // method-3 (only get)
//! for v in nums.iter() {
//!     print!("{} ", v);
//! }
//!
//! // change elements of array or vector
//! // method-4 (can set)
//! for v in &mut nums {
//!     *v += 2;
//! }
//!
//! // method-5 (can set)
//! for v in nums.iter_mut() {
//!     *v += 2;
//! }
//! ```
//!
//!




fn example() {
    /// init
    let mut nums = [-5, 4, 1, 32, -3, 2];
    // let mut nums: [i32; 5] = [-5, 4, 1, 32, -3, 2];
    // let mut nums = vec![-5, 4, 1, 32, -3, 2];
    // let mut nums: Vec<i32> = vec![-5, 4, 1, 32, -3, 2];

    /// sort
    // nums.sort();                           // [-5, -3, 1, 2, 4, 32]
    // nums.sort_by(|a, b| a.cmp(b));         // [-5, -3, 1, 2, 4, 32]
    /// reverse sort
    // nums.sort_by(|a, b| b.cmp(a));         // [32, 4, 2, 1, -3, -5]
    /// sort by string
    // nums.sort_by_cached_key(|k| k.to_string());      // [-3, -5, 1, 2, 32, 4]
    /// shuffle
    use rand::prelude::*;
    nums.shuffle(&mut rand::thread_rng());          // [4, -3, 2, -5, 32, 1]

    // println!("{:?}", nums.to_vec());
    // println!("{:?}", nums);
    //
    type_of(&nums);                           // [i32; 6]
    type_of(&[-5, 4, 1, 32, -3, 2]);          // [i32; 6]
    type_of(&32.90);                          // prints "f64"
    type_of(&vec![1, 2, 4]);                  // prints "std::vec::Vec<i32>"
    type_of(&"foo");                          // prints "&str"
}


fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>() );
}