use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Bob";
    // println!("Command: {}", command);
    // cargo run hello world
    // Args: ["target\\debug\\rust-playground.exe", "hello", "world"]

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    }
}
