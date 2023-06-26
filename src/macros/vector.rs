#[macro_export]
macro_rules! vector {
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


pub fn example() {
    let v = vector![1, 2, 3];
    println!("vec {:?}", v);
}
// vec [1, 2, 3]