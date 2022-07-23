pub fn run() {
    // print to console
    println!("Hello, World");

    // Basic Formatting
    println!("{} is from {}", "Brad", "Mass");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Join", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    println!("10 + 10 = {}", 10 + 10);
}






