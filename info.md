
# Rust Input Handling and Graceful Error Handling

## Basic Input Handling in Rust
Rust provides input handling through the `std::io` module. Input from the user is read as a `String` and can be parsed into other data types as needed.

### Reading a Line of Input
1. **Create a mutable `String`** to store the user input.
2. **Use `io::stdin().read_line()`** to read input and store it in the `String`.
3. **Handle potential errors** from `read_line()` using `.expect()` or other error handling techniques.

#### Example:
```rust
use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter some text:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You entered: {}", input);
}
```

- `expect("Failed to read line")` is a basic error handling technique. If `read_line` fails, it will print an error message and stop the program.

### Trimming Whitespace
- Use `input.trim()` to remove any extra whitespace or newline characters from the input.

#### Example:
```rust
let trimmed_input = input.trim();
println!("Trimmed input: {}", trimmed_input);
```

---

## Converting Input to Other Data Types
Rust initially reads input as a `String`. To use it as another data type (e.g., `i32`, `f64`), convert the input using `.parse()`.

### Parsing Input
1. **Trim the input** to remove whitespace.
2. **Use `.parse::<Type>()`** to convert the input to the desired type.
3. **Handle errors** if the input does not match the expected type.

#### Example: Parsing to Integer
```rust
use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = input.trim().parse()
        .expect("Please enter a valid number");

    println!("You entered: {}", number);
}
```

- In this example, `input.trim().parse()` attempts to convert the trimmed input to an integer (`i32`). If parsing fails, the program stops with a message.

---

## Graceful Error Handling
Graceful error handling allows the program to handle errors without crashing, providing clear feedback to the user.

### Using `match` for Graceful Error Handling
Instead of using `.expect()`, use a `match` statement to handle errors in a more user-friendly way.

#### Example:
```rust
use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter a number:");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: Result<i32, _> = input.trim().parse();

    match number {
        Ok(num) => println!("You entered: {}", num),
        Err(_) => println!("That’s not a valid number! Please try again."),
    }
}
```

- **Explanation**:
  - `Ok(num)` executes if parsing is successful, printing the parsed number.
  - `Err(_)` executes if parsing fails, printing a helpful error message instead of crashing.

### Why Use Graceful Error Handling?
- **Improves User Experience**: Clear, helpful messages improve interaction.
- **Prevents Crashes**: The program can continue running even if minor errors occur.
- **Customizable Responses**: Provides flexibility in handling different types of errors.

---

## Summary
1. **Read Input**: Use `read_line()` to read user input as a `String`.
2. **Parse Input**: Convert input to other types using `.parse()`, handling errors gracefully.
3. **Graceful Error Handling**: Use `match` statements for error handling, providing clear messages and maintaining program flow.

### Example: Complete Input and Error Handling
```rust
use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter a number:");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<i32>() {
        Ok(num) => println!("You entered: {}", num),
        Err(_) => println!("That’s not a valid number! Please try again."),
    }
}
```

This approach to input and error handling in Rust creates a robust, user-friendly program that responds effectively to both valid and invalid inputs.
```
