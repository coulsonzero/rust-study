use crate::control::for_loop::*;

#[test]
pub fn test_for() {

    for_str("hello world");
    for_str(&String::from("hello world"));
    // output: { 'h' 'e' 'l' 'l' 'o' ' ' 'w' 'o' 'r' 'l' 'd' }

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    for_array(arr.as_mut());
    for_array(vector.as_mut_slice());

    // output: [1, 2, 3, 4, 5]
}