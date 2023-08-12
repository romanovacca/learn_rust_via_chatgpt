

```let mut tasks = Vec::new();```

Returns the error

```
help: consider giving `tasks` an explicit type, where the type for type parameter `T` is specified
  |
2 |     let mut tasks: Vec<T> = Vec::new();
  |                  ++++++++
```

Reason for the error:
The error occurs because Rust's type inference is unable to determine the type of elements that the Vec should contain.

When you use `Vec::new()` without any initial elements, Rust doesn't have enough information to infer the element type, so you need to explicitly specify the type of elements the vector will hold.

To resolve this, specify the type of elements that the vector will hold when declaring it. For example, if you're creating a vector to hold strings, you can explicitly specify the type as String:

```
let mut tasks: Vec<String> = Vec::new();
```

 
--

Running `println!("Items in list are: {}",tasks);` where tasks is a `Vector` gives the following error:

```
error[E0277]: `Vec<String>` doesn't implement `std::fmt::Display`
  --> src/main.rs:29:38
   |
29 |     println!("Items in list are: {}",tasks);
   |                                      ^^^^^ `Vec<String>` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Vec<String>`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```

To print the contents of a Vec<String> using println!, you need to iterate over the vector's elements and format them appropriately. One common way to do this is to use the .join() method to concatenate the elements into a single string.

```
println!("Items in list are: {}", tasks.join(", "));
```

--

