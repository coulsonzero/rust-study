//! struct.func()

pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("The area of the rectangle is {} square pixels.", area(&rect1));
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };

    let ans = rect1.can_hold(&rect2);
    println!("{}", ans);    // false

    let rect = Rectangle::new(20, 12);
    println!("{} {}", rect.height, rect.width);
}


struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn new(w: u32, h: u32) -> Rectangle {
        return Rectangle {  width: w,  height: h };
    }

    fn area(&self) -> u32 {
        return self.width * self.height
    }


    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}