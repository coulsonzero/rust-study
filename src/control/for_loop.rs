//! for loop:
//!
//! ```
//! (1) str || string:
//!         a) for c in s.chars() {...}
//!         b) for (_, c) in s.chars().enumerate() {...}
//! (2) array || vector:
//!         for i in 0..nums.len() {}
//!         for v in nums {}
//!         for v in nums.iter() {}
//!         for v in &mut nums {}
//!         for v in nums.iter_mut() {}
//! (3) hashmap:
//!         for (k, v) in &map {}
//! ```









// &str || String
pub fn for_str(s: &str) {
    print!("|\x1b[96m{:?}\x1b[0m| => {{ ", s);
    for (_, c) in s.chars().enumerate() {
        print!("{:?} ", c);
    }
    println!("}}");
}


// array || vector
pub fn for_array(nums: &mut [i32]) {
    print!("|\x1b[96m{:?}\x1b[0m| => ", nums);
    print!("[ ");
    for i in 0..nums.len() {
        print!("{}", nums[i]);
        if i != nums.len()-1 {
            print!(", ");
        }
    }
    println!(" ]");
}

// use std::collections::hash_map::RandomState;
// use std::collections::HashMap;
// pub fn for_hashmap(map: HashMap<K, V, RandomState>) {
//
// }