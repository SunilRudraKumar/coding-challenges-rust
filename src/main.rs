use std::io;

enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operations {
    fn from_str(op: &str) -> Option<Operations> {
        match op {
            "+" => Some(Operations::Add),
            "-" => Some(Operations::Subtract),
            "*" => Some(Operations::Multiply),
            "/" => Some(Operations::Divide),
            _ => None,
        }
    }
}

fn main() {
    println!("Welcome to the CLI Calculator made with Rust");
    println!("Please enter the first value:");

    let mut first_value = String::new();
    io::stdin()
        .read_line(&mut first_value)
        .expect("Failed to read line");

    let num1: i32 = match first_value.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    println!("Please enter the operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    let op = match Operations::from_str(operation.trim()) {
        Some(op) => op,
        None => {
            println!("Invalid operation entered. Please use +, -, *, or /.");
            return;
        }
    };

    println!("Please enter the second value:");
    let mut second_value = String::new();
    io::stdin()
        .read_line(&mut second_value)
        .expect("Failed to read line");

    let num2: i32 = match second_value.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    let result = match op {
        Operations::Add => add(num1, num2),
        Operations::Subtract => subtract(num1, num2),
        Operations::Multiply => multiply(num1, num2),
        Operations::Divide => divide(num1, num2),
    };

    println!("The result is: {}", result);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn subtract(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn divide(num1: i32, num2: i32) -> i32 {
    num1 / num2
}
