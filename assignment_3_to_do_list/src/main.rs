use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    println!("Welcome to the Rust Todo List!");

    loop {
        println!("Available commands: add, list, quit");

        println!("Enter a command:");
        let mut input_command = String::new();
        io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line");
        let command = input_command.trim();

        match command {
            "add" => {
                println!("Enter task description:");
                let mut input_description = String::new();
                io::stdin()
                    .read_line(&mut input_description)
                    .expect("Failed to read line");
                let command_description = input_description.trim();

                tasks.push(command_description.to_string());
                println!("Task added");
            }
            "list" => {
                println!("Items in list are: {}", tasks.join(", "));
            }
            "quit" => {
                println!("Shutting down");
                break;
            }
            _ => {
                panic!("Invalid command: {command}");
            }
        };
    }
}
