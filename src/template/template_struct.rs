struct Point<T> {
    x: T,
    y: T,
}

pub fn run() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let int_float = Point { x: 5, y: 4.0 as i32 };
}