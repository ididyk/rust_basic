use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let command = args[1].clone();
    // println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, How are you?", "Valera");
    } else if command == "status" {
        println!("Status is 100%");
    }else {
        println!("Unknown command");
    }

}