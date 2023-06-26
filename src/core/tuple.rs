pub fn run () {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let a: i32 = x.0;
    let b: f64 = x.1;
    let c: u8  = x.2;

    println!("{:?}", (a, b, c));
}