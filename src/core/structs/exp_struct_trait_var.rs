#[warn(unused)]
trait Num {
    fn _abs(self) -> f64;
}

impl Num for f64 {
    fn _abs(self) -> f64 {
        return if self < 0.0 {-self} else {self}
    }
}


#[test]
fn test_abs() {
    let num1: f64 = -12.3;
    let num2: i32 = -27;

    println!("{}", num1 as i32);         // -12
    println!("{}", num1._abs());          // 12.3
    println!("{}", (num2 as f64)._abs()); // 27

}