fn max<T: PartialOrd>(a: T, b: T) -> T {
    return if a > b {a} else {b};
}


fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest
}



#[test]
fn test_max() {
    println!("{:?}", max(3, 7));    // 7
}

#[test]
fn test_largest() {
    let vec_nums = vec![34, 50, 25, 100, 65];
    let vec_char = vec!['y', 'm', 'a', 'q'];
    println!("output: {:?}", largest(&vec_nums));     // 100
    println!("output: {:?}", largest(&vec_char));     // 'y'
}