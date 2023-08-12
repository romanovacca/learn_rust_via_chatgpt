Assignment 1:

Rust is a systems programming language that focuses on safety, performance, and concurrency. It's known for its strict memory safety rules enforced by the borrow checker. Let's start with a bit of theory before we dive into an exercise.

Ownership and Borrowing in Rust:
In Rust, the concept of `ownership` ensures that memory is managed safely and efficiently. Each value in Rust has a variable that's its `"owner."` There can only be one owner at a time. When the owner goes out of scope, Rust automatically deallocates the memory associated with the value.

`Borrowing` is another crucial concept. You can pass references to values to other parts of your code, either as mutable or immutable references. These references are subject to the rules of the borrow checker, which ensures that you can't have data races or invalid memory access.

In Rust, the `&` symbol is used to create references to values rather than copying the values themselves. A `reference` is a lightweight way to refer to a value without taking ownership of it. References are essential in Rust because they allow you to pass data to functions without moving ownership, enabling efficient and safe memory usage.

Exercise: Convert a Python Function to Rust
Let's start with a simple exercise to get you started. We'll convert a basic Python function to Rust. Here's the Python code:

```
def calculate_sum_square(numbers):
    total = 0
    for num in numbers:
        total += num * num
    return total

nums = [1, 2, 3, 4, 5]
result = calculate_sum_square(nums)
print(result)
```

Your task is to convert this code into Rust. Follow these steps:

Declare the calculate_sum_square function.
Use a loop to iterate over the numbers.
Calculate the sum of squares.
Return the result.
Assignment: Convert and Execute
Now, let's get hands-on. Convert the given Python code to equivalent Rust code. Create a Rust project and write the necessary code. After that, compile and execute your Rust program.

Answer:
Here's how you can implement the Python function in Rust:

Create a new Rust project using cargo new rust_intro.
Navigate to the src directory within the project folder and open the main.rs file.

Run cargo run to execute the program. You should see the output printed to the console.
This exercise will help you get familiar with Rust's syntax, ownership model, and basic constructs. Feel free to explore more complex exercises and concepts as you continue your Rust journey!