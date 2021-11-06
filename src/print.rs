pub fn run() {
    // This is a comment
    // Print
    println!("Hello from the print.rs file!");

    // Variables in printing
    println!("Printing variables: {}", "12345");

    // Multiple variables in printing
    println!(
        "Multiple Variables!\nVariable 1: {}\nVariable 2: {}",
        "1234", "5678"
    );

    // Positional variables in printing
    println!(
        "Positional Variables:\n{0} is from {1} and {0} likes to {2}",
        "Bob", "Mars", "Code"
    );

    // Named variables in printing
    println!(
        "Named Variables:\n{name} likes to {hobby}",
        name = "Bob",
        hobby = "Code"
    );
}
