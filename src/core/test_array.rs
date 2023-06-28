#[test]
fn test_iter_array() {
    let mut nums: [i32; 7] = [3, 7, 5, 12, 1, 9, 4];
    let mut nums: Vec<i32> = vec![-5, 4, 1, 32, -3, 2];

    /// get or change value
    for i in 0..nums.len() {
        // print!("{} ", nums[i]);
        nums[i] += 1;
    }

    /// only get
    // for v in nums {
    //     print!("{} ", v);
    // }

    // for v in nums.iter() {
    //     print!("{} ", v);
    // }

    /// change value
    // for v in &mut nums {
    //     *v += 2;
    // }

    // for v in nums.iter_mut() {
    //     *v += 2;
    // }

    /// output
    println!("{:?}", nums);
}
