pub fn run() {
    let mut hello = String::from("Hello");

    // Print
    println!("{}", hello);

    // Get length
    println!("String Length: {}", hello.len());

    // Push into string
    hello.push_str(", World!");
    println!("After push: {}", hello);

    // Check if string is empty
    let is_empty = hello.is_empty();
    println!("String is empty: {}", is_empty);

    // Check if string contains parameters
    let contains_world = hello.contains("World");
    println!("String contains 'World': {}", contains_world);

    // Replace string
    let replaced_hello = hello.replace("World", "There");
    println!("{}", replaced_hello);

    // Loop through string and print separately by whitespace/spacing
    println!("Split by whitespace for loop:");
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
}
