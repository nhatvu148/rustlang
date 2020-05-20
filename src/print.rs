pub fn run() {
    // Print to console
    println!("Hello! from the print.rs file");

    // Basic formatting
    println!("Number {} is number of {}", 1, "Vu");

    // Positional arguments
    println!("{0} is from {1} and {0} is {2}!", "Vu", "Vietnam", "cool");

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Vu",
        activity = "soccer"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}.", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hi there!"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
