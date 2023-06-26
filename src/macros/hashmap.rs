pub fn example() {
    test_hashmap();
}



#[macro_export]
macro_rules! hashmap {
    ($($key: expr => $val: expr), *) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(
                temp_map.insert($key, $val);
            )*
             temp_map
        }
    };
}




// #[test]
fn test_hashmap() {
    let map = hashmap!(1 => "one", 2 => "two", 3 => "three" );
    println!("map {:?} ",map);
}
// map {2: "two", 3: "three", 1: "one"}



