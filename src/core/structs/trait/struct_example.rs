/// similar function for different struct

/*
//! struct ... for ...
trait Noise {
    fn make_noise(&self);
}

struct Person;
impl Noise for Person {
    fn make_noise(&self) {
        println!("hello");
    }
}

struct Cat;
impl Noise for Cat {
    fn make_noise(&self) {
        println!("woof");
    }
}
 */



//! trait
trait Noise {
    fn make_noise(&self);
}

fn hello(noisy: impl Noise) {
    noisy.make_noise();
}

fn main() {
    hello(Person {} );
    hello(Cat {} );
}

