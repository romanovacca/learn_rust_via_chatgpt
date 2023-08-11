use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");

    println!("Enter the first number:");
    let mut input_1 = String::new();
    io::stdin()
        .read_line(&mut input_1)
        .expect("Failed to read line");
    let first_number: i32 = input_1.trim().parse().expect("Not a valid number");

    println!("Enter the second number:");
    let mut input_2 = String::new();
    io::stdin()
        .read_line(&mut input_2)
        .expect("Failed to read line");
    let second_number: i32 = input_2.trim().parse().expect("Not a valid number");

    println!("Enter an operator");
    let mut input_3 = String::new();
    io::stdin()
        .read_line(&mut input_3)
        .expect("Failed to read line");
    let operator = input_3.trim();
    // println!("{:?}",operator);

    let result = match operator {
        "-" => first_number - second_number,
        "+" => first_number + second_number,
        "*" => first_number * second_number,
        "/" => first_number / second_number,
        _ => {
            panic!("Invalid operator: {operator}");
        }
    };

    println!("Result: {first_number} {operator} {second_number} = {result}");
}
