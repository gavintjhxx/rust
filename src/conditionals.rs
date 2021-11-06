pub fn run() {
    let age: i32 = 21;
    let check_id: bool = true;

    // IF ELSE statement
    if age >= 21 && check_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: You're under 18.");
    } else {
        println!("Bartender: Can I have your ID?");
    }

    // Shorthand if, similar to Javascript 'statement ? true : false'
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {:?}", is_of_age);
}
