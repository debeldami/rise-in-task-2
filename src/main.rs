use std::io;

// Define an enum to represent different mathematical operations.
enum Operator {
    Add(f64, f64),
    Substract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Implement a method to perform the calculation based on the operator.
fn calculate(value: Operator) -> f64 {
    match value {
        Operator::Add(a, b) => {
            return a + b;
        }
        Operator::Substract(a, b) => {
            return a - b;
        }
        Operator::Multiply(a, b) => {
            return a * b;
        }
        Operator::Divide(a, b) => {
            return a / b;
        }
    }
}

// Get the user's chosen operator.

fn get_user_operator(first_value: f64, second_value: f64) -> Operator {
    loop {
        println!("Enter an operator: +, -, *, or /");

        let result = get_user_input();

        match result.as_str() {
            "+" => return Operator::Add(first_value, second_value),
            "-" => return Operator::Substract(first_value, second_value),
            "*" => return Operator::Multiply(first_value, second_value),
            "/" => return Operator::Divide(first_value, second_value),
            _ => {
                println!("Enter a valid operator: +, -, *, or /");
            }
        }
    }
}

// Get user input as a string.
fn get_user_input() -> String {
    let mut user_input = String::new();

    let stdin = io::stdin(); // We get `Stdin` here.

    stdin
        .read_line(&mut user_input)
        .expect("Failed to read input");

    user_input.trim().to_string()
}

// Get a numeric value from the user.
fn get_user_value(comment: &str) -> f64 {
    println!("{}", comment);

    loop {
        let result: Result<f64, _> = get_user_input().parse();

        match result {
            Ok(usize_value) => {
                return usize_value;
            }
            Err(_) => {
                println!("{}", comment);
            }
        }
    }
}

fn main() {
    let first_value = get_user_value("Enter first value");
    let second_value = get_user_value("Enter second value");
    let operator = get_user_operator(first_value, second_value);

    let ans = calculate(operator);

    println!("Your answer is {}", ans);
}
