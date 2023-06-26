// Loops - Used to interate until a condition is met

pub fn run() {
    loop_example();
    while_loop_example();
    for_range_example();
}



fn for_range_example() {
    // For Range
    for x in 0..6 {
        print!("{} ", x);
    }
    println!();
}
// 0 1 2 3 4 5


// Infinite loop
fn loop_example() {
    let mut count: i32 = 0;
    loop {
        print!("{} ", count);
        count += 1;
        if count == 6 {break};
    }
    println!();
}
// 0 1 2 3 4 5


// While loop
fn while_loop_example() {
    let mut count: i32 = 0;
    while count < 6 {
        print!("{} ", count);
        count += 1;
    }
    println!();
}
// 0 1 2 3 4 5