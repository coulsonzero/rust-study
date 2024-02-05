#[test]
fn example() {
    let day: i32 = 3;
    let res = match day {
        1 | 2 | 3 | 4 | 5 => "Workday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "default",
    };
    println!("{:?}", res);    // Workday

    println!("{:?}", match day {
        1..=5 => "Workday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "default",
    });
}

#[test]
fn test_char() {
    let c = '2';
    let res: &str = match c {
        '1'..='9' => "integer",
        'a'..='z' => "lower letter",
        'A'..='Z' => "upper letter",
        _ => "default",
    };
    println!("{:?}", res);
}



fn run() {
    let c = 'f';
    let valid_variable = match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        'α'..='ω' => true,
        _ => false,
    };

    let ph = 10;
    println!("{}", match ph {
        0..=6 => "acid",
        7 => "neutral",
        8..=14 => "base",
        _ => unreachable!(),
    });



    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);



    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }


    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }


    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }



    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}