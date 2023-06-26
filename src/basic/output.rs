/// output
///



pub fn run() {
    println!("hello, world!");

    // format
    println!("hello {}", "John Smith");
    println!("{} is located in {}", "Shanghai", "China");

    // Positional Arguments
    println!("{1} is located in {2}, and my name is {0}, ", "John Smith", "Shanghai", "China");

    // Named arguments
    println!("{city} is located in {country}", city = "Shanghai", country = "China");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);     // Binary: 1010 Hex: a Octal: 12

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));      // (12, true, "hello")


    println!("|{:<16}|:", 3.14156);     // |3.14156         |:
    println!("|{:^16}|:", 3.14156);     // |    3.14156     |:
    println!("|{:>16}|:", 3.14156);     // |         3.14156|:
    println!("{:05}", 3.1);             // 003.1

    println!("{:.2}", 3.14156);                   // 3.14
    println!("{:.1$}", 3.14156, 2);               // 3.14
    println!("{1:.0$}", 2, 3.14156);              // 3.14
    println!("{:.width$}", 3.14156, width = 2);   // 3.14
}