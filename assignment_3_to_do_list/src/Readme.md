Assignment: Build a Rust Todo List

Objective:
Develop a Rust command-line application that empowers users to manage their tasks using a todo list. 

The program should allow task addition, listing existing tasks, and quitting the application.

New Theory:

Vectors (Vec): Vectors are dynamic arrays capable of storing and manipulating multiple values. We'll employ a vector to maintain the task list efficiently.

A vector, declared as:
```
let mut tasks: Vec<String> = Vec::new();
```
can store strings dynamically. You can use .push() to add items and .iter() to iterate over its content. Here's a snippet showcasing vector usage:

```
use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    tasks.push("Buy groceries".to_string());
    tasks.push("Clean the house".to_string());

    println!("Tasks:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{}. {}", index + 1, task);
    }
}

```

Instructions:

1. Setting Up the Project:

    Create a fresh Rust project named "todo_list" using the command 
    `cargo new todo_list`.

2. Crafting the Todo List:

* Harness a Vec<String> to store tasks input by users.
* Construct a loop to repeatedly prompt users for actions: adding a task, listing tasks, or exiting.
* Employ a match expression to accommodate varying user actions.

3. Adding Tasks:

* Within the match expression, address the "add" case.
* Solicit users for a task description using println! and io::stdin().
* Exploit the push method to append the task description to the vector.
4. Listing Tasks:

* Within the match expression, tackle the "list" case.
* Leverage a for loop to traverse the vector, displaying each task using println!.

5. Exiting the Application:

* Inside the match expression, handle the "quit" case.
* Employ the break statement to exit the loop gracefully.

6. Handling Invalid Commands:

* Manage any other input with a message declaring "Invalid command."

Example Usage:

```
Welcome to the Rust Todo List!
Available commands: add, list, quit

Enter a command: add
Enter task description: Buy groceries

Enter a command: add
Enter task description: Clean the house

Enter a command: list
Tasks:
1. Buy groceries
2. Clean the house

Enter a command: quit
Goodbye!
```

Remember, the goal is to utilize a vector for managing tasks efficiently and to implement the new concepts while creating your todo list application.