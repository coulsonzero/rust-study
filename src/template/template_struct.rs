struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn new(x: X1, y: Y1) -> Point<X1, Y1> {
        return Point { x,  y}
    }

    fn mix<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        return Point { x: self.x, y: other.y }
    }
}

#[test]
fn test_struct_temp() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let int_float = Point { x: 5, y: 4.0 as i32 };

    let p = Point { x: 5, y: 10 };
    println!("point: ({}, {})", p.x, p.y);    // point: (5, 10)

    let p = Point::new(3, 6);
    println!("point: ({}, {})", p.x, p.y);    // point: (3, 6)


    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mix(p2);
    println!("point: ({:?}, {:?})", p3.x, p3.y);    // point: (5, 'c')
}