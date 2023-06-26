//! # Vector
//!
//! init:
//! ```rust
//! let mut nums = Vec::new();
//! let mut nums: Vec<i32> = Vec::new();
//! let nums = vec![1, 2, 3, 4];
//! let mut nums: Vec<i32> = vec![1,2,3,4];
//! let mut nums: Vec<i32> = (1..10).collect();
//! ```
//!
//! methods
//! ```rust
//! 增：push(e)
//! 删：pop()
//! 查：nums[i], &nums[i], &nums[0..2]
//! 改：nums[i] = v
//!
//! 长度：len()
//! 打印：
//! println!("nums: {:?}", nums);
//! println!("nums: {:?}", &nums);
//!
//! nums.sort();
//! nums.shuffle(&mut rng);
//! ```
//! 遍历：
//! ```rust
//! // not mut
//! let nums = vec![100, 32, 57];
//! print!("[ ");
//! for v in &nums {
//!     print!("{} ", v);
//! }
//! println!("]");
//! ```
//!
//! ```rust
//! // if need change: mut
//! let mut nums = vec![100, 32, 57];
//! for v in &mut nums {
//!     *v += 2;
//! }
//! println!();
//! ```
//!
//! ```rust
//! print!("[ ");
//! for v in nums.iter() {
//!     print!("{} ", v);
//! }
//! println!("]");
//! ```
//! ```rust
//! // Loop & mutate values
//! for x in nums.iter_mut() {
//!     *x *= 2; // multiply by 2
//! }
//! ```


use std::mem;

pub fn run() {
    // let mut nums: Vec<i32> = Vec::new();
    //
    // nums.push(5);
    // nums.push(6);
    // nums.push(7);
    // nums.push(8);
    //
    // println!("nums[2] is {}", &nums[2]);

    let mut nums = vec![100, 32, 57];
    for v in &mut nums {
        *v += 2;
    }
    println!();
    println!("v: {:?}", nums);
    println!("v: {:?}", &nums[0..2]);
}


fn example() {
    let mut nums: Vec<i32> = vec![1,2,3,4];

    // Re-assign values
    nums[2] = 20;

    // Add to vector
    nums.push(5);
    nums.push(6);

    // Pop off last value
    nums.pop();

    // Get first value
    println!("First value: {}", nums[0]);

    // Get vector length
    println!("Vector length: {}", nums.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&nums));

    let slice: &[i32] = &nums[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    print!("nums: [ ");
    for v in nums.iter() {
        print!("{} ", v);
    }
    println!("]");

    // Loop & mutate values
    for x in nums.iter_mut() {
        *x *= 2; // multiply by 2
    }

    println!("Numbers Vec: {:?}", nums);

}