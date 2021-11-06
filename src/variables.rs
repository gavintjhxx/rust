pub fn run() {
    let name = "Bob";
    // Age needs to be mutable for changing values in future
    let mut age = 1;
    println!("My name is {name} and I am {age}", name = name, age = age);
    // Changing variable values
    age = 0;
    println!("My name is {name} and I am {age}", name = name, age = age);

    // Constants needs type specs
    const ID: i32 = 1;
    println!("ID: {id}", id = ID);

    // Multiple variables
    let (my_name, my_age) = ("Bob", 1);
    println!(
        "My name is {myName} and I am {myAge}",
        myName = my_name,
        myAge = my_age
    );
}
