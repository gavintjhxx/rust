pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);

    // Reassign value
    numbers[2] = 10;
    println!("Number: {:?}", numbers);

    // Print specific value
    println!("Second Number: {:?}", numbers[2]);

    // Get array Length
    println!("Array length: {}", numbers.len());

    // Memory space
    println!(
        "Memory space occupied by array (bytes): {}",
        std::mem::size_of_val(&numbers)
    );

    // Get slice
    let sliced: &[i32] = &numbers[0..2];
    println!("Array slice: {:?}", sliced);
}
