Assignment 2: Build a Simple Rust Calculator

Objective: In this assignment, you'll build a basic calculator program in Rust that performs simple arithmetic operations like addition, subtraction, multiplication, and division.

This exercise will guide you through asking for user input, making decisions in code, and displaying results.

New Theory:

A `match` expression is a way to handle different cases or patterns in Rust. In this case, we use the match expression to handle the different operators the user might enter.

Here's how it works:

We start by using match operator to compare the operator string the user entered to different patterns (in this case, the operators).
For each pattern, we provide the corresponding calculation or action.
For instance, this part of the code handles the addition operator (+):

Instructions:

1. Setting Up the Project:

    Begin by creating a new Rust project named "calculator" using the command cargo new calculator.

2. Gathering User Input:

    To gather information from the user, you'll need to import a specific part of Rust's standard library. Use the following line at the beginning of your src/main.rs file:
    
    ```
     use std::io; 
    ```
    
    Next, you'll set up a way to accept input from the user. To do this, use the following code snippet:

    ```
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    ```
    
    This code creates a new, mutable variable input and then uses `io::stdin().read_line(&mut input)` to read a line of text from the user and store it in the input variable.

3. Performing Calculations:

    Now that you have the user's input, you can process it. Prompt the user to enter two numbers and an operator (+, -, *, /), and then use the input variable to store their responses.

    To perform calculations, you'll need to convert the user's input to numbers. You can do this using the .parse() method on strings. For instance, if you have a string number_str, you can convert it to an integer like this:

    ```
    let number: i32 = number_str.trim().parse().expect("Not a valid number");
    ```
    Use Rust's `match` expression to determine which operator the user entered and perform the corresponding calculation.

4. Displaying the Result:

    Once the calculation is done, you can display the result to the user using the `println!`` macro.

    Example Output:
    ```
    Welcome to the Rust Calculator!
    Enter the first number: 10
    Enter the second number: 5
    Enter an operator (+, -, *, /): +
    Result: 10 + 5 = 15
    ```