pub trait Descriptive {
    fn describe(&self) -> String;
}


struct Person {
    name: String,
    age: u8
}

struct Student {
    supper: Person,
    score: f64
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

impl Descriptive for Student {
    fn describe(&self) -> String {
        format!("{} {} {}", self.supper.name, self.supper.age, self.score)
    }
}

#[test]
fn test_trait() {
    let p = Person { name: String::from("Cali"), age: 24 };
    println!("{:?}", p.describe());     // "Cali 24"

    let s = Student { supper: p, score: 97.5 };
    println!("{:?}", s.describe());     // "Cali 24 97.5"
}

