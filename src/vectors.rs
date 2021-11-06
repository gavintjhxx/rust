// Vectors are resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);

    // Add number
    numbers.push(6);
    println!("Number after adding: {:?}", numbers);

    // Remove last number
    numbers.pop();
    println!("Number after removing: {:?}", numbers);

    // Reassign value
    numbers[2] = 10;
    println!("Number: {:?}", numbers);

    // Print specific value
    println!("Second Number: {:?}", numbers[2]);

    // Get vector Length
    println!("Vector length: {}", numbers.len());

    // Memory space
    println!(
        "Memory space occupied by vector (bytes): {}",
        std::mem::size_of_val(&numbers)
    );

    // Get slice
    let sliced: &[i32] = &numbers[0..2];
    println!("Vector slice: {:?}", sliced);

    // Loop through vector values
    println!("Looping through vector values");
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop through and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers after mutate: {:?}", numbers);
}
