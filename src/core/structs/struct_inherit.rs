
struct Animal {
    name: String,
    age: i32
}

struct Dog {
    supper: Animal,     /// inherit
    type_name: String
}

impl Animal {
    fn new(name: String, age: i32) -> Self {
        return Animal{ name, age };
    }
    fn speak(&self) {
        println!("I'm an animal. Age = {}", self.age);
    }
}

impl Dog {
    fn new(name: String, age: i32, type_name: String) -> Self {
        return Dog{ supper: Animal { name, age }, type_name }
    }
    /// override method
    fn speak(&self) {
        self.supper.speak();
        println!("type_name = {}", self.type_name);
    }
}


#[test]
fn test_inherit() {
    let dog = Dog::new(
        String::from("tom"),
        2,
        String::from("labuladuo")
    );

    dog.speak();
    // I'm an animal. Age = 2
    // type_name = labuladuo
}
