// Structs - used to store custom data types
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn run() {
    // Traditional struct
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };
    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    //

    // Tuple struct
    // let mut c = Color(255, 0, 0);
    // println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("John", "Doe");
    println!("Person {}", p.full_name());
}
