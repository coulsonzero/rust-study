pub fn run() {
    random_example();
    // thread_example();
}


fn random_example() {
    use rand::Rng;


    println!("[0-9]: {:?}",rand::thread_rng().gen_range(0..10));          // [0-9]:  4
    println!("[0-9]: {:?}",rand::thread_rng().gen_range(0..=10));         // [0-10]: 4
    println!("[a-z]: {:?}",rand::thread_rng().gen_range('a'..'z'));       // [a-z]: 's'
    println!("[A-Z]: {:?}",rand::thread_rng().gen_range('A'..'Z'));       // [A-Z]: 'V'

    println!("i32: {}", rand::random::<i32>());          // i32: -512141323
    println!("f64: {}", rand::random::<f64>());          // f64: 0.7152522484866201
    println!("char: {:?}", rand::random::<char>());      // char: '\u{a951c}'

    println!("char: {:?}",rand::thread_rng().gen::<char>());     // char: 'V'
    println!("i32: {:?}",rand::thread_rng().gen::<i32>());       // i32: -876188872
    println!("f64: {:?}",rand::thread_rng().gen::<f64>());       // f64: 0.3292520239508353


}

fn thread_example() {
    use rand::prelude::SliceRandom;     // shuffle

    let mut nums: Vec<i32> = (1..10).collect();     // [1, 2, 3, 4, 5, 6, 7, 8, 9]
    nums.shuffle(&mut rand::thread_rng());     // [1, 3, 7, 4, 8, 6, 2, 9, 5]
    println!("{:?}", nums);

}


