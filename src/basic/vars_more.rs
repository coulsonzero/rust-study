pub fn run() {
    // let：绑定变量，mut申明可变，具有shadowing特性
    let x: i32 = 12;
    let mut y: i32 = 20;
    // const：常量
    const PI: f64 = 3.14159;
    // static：绑定静态全局变量，指向同一个内存地址，mut声明可变，操作修改需使用unsafe.
    static mut Z: i32 = 0;  // warning: static variable `z` should have an upper case name
    unsafe {
        Z += 1;
        println!("{}", Z);
    }

    // 多个变量声明
    let (name, age) = ("John Smith", 21);

    println!("{}, {}, {}", x, y, PI);
    println!("{:?}", (name, age));
}



pub fn let_example() {
    let x: i32 = 12;
    // println!("{:p}", &x);
    let x: f64 = 3.14;
    // println!("{:p}", &x);
    let x: char = 'k';
    let x: &str = "John Smith";

    println!("{}", x);
}