use std::collections::HashMap;


pub fn run() {
    let mut map: HashMap<String, i32> = HashMap::new();

    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 50);

    // unordered
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
    // Yellow: 50
    // Blue: 10


    // ordered
    println!("{:?}", map);
    // output: {"Blue": 10, "Yellow": 50}
}


fn example() {
    use std::collections::HashMap;

    let map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
    ]);

    // map.values_mut(), map.keys()
    for val in map.values() {
        println!("{}", val);
    }
}
