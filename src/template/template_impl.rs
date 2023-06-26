struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x
    }
}

pub fn run() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

// output: p.x = 5
