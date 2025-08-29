use std::io::{self};

#[derive(Debug, Clone, Copy)]
enum Number {
    Int(i32),
    Float(f64),
}

fn integer_input(position: &str) -> Option<Number> {
    let mut buf = String::new();
    println!("Input {position} int value");
    if io::stdin().read_line(&mut buf).is_err() {
        println!("Error in reading from console");
        return None
    }
    let trimmed = buf.trim();

    if let Ok(val) = trimmed.parse::<i32>(){
        return Some(Number::Int(val));
    }

    if let Ok(val) = trimmed.parse::<f64>(){
        return Some(Number::Float(val));
    }
    println!("This value is not valid number\n");
    None
}

fn operation_input() -> Option<String> {
    println!("Input some operation");
    let mut operation = String::new();
    match io::stdin().read_line(&mut operation) {
        Ok(_) => {
            if matches!(operation.trim(), "+" | "-" | "*" | "/"){
                Some(operation.trim().to_string())
            } else {
                println!("Not valid operation\nPlease select one of this + - * /");
                None
            }
        },
        Err(_) => {
            println!("Something went wrong");
            None
        },
    }
}

fn make_operation(first: Number, second: Number, operation: &str) -> Option<Number> {
    let (firstf,secondf) = match (first, second) {
        (Number::Int(first), Number::Int(second)) => (first as f64, second as f64),
        (Number::Int(first), Number::Float(second)) => (first as f64, second),
        (Number::Float(first), Number::Int(second)) => (first, second as f64),
        (Number::Float(first), Number::Float(second)) => (first, second),
    };
    let res = match operation.trim() {
        "-" => firstf - secondf,
        "+" => firstf + secondf,
        "*" => firstf * secondf,
        "/" => {
            if secondf == 0.0 {
                println!("Division by zero!");
                return None;
            }
            firstf / secondf
        },
        _ => {
            println!("Unknown operation\n");
            return None;
        },
    };
    if res.fract() == 0.0 {
        Some(Number::Int(res as i32))
    } else {
        Some(Number::Float(res))
    }
}

fn main() {
    let first = loop {
        if let Some(val) = integer_input("first"){
            break val
        } else {
            println!("This value is not valid integer\n")
        }
    };
    let second = loop {
        if let Some(val) = integer_input("second"){
            break val
        } else {
            println!("This value is not valid integer\n")
        }
    };
    let operation = loop {
        if let Some(val) = operation_input(){
            break val
        }
    };
    match make_operation(first, second, &operation) {
        Some(Number::Int(i)) => println!("Result is {i}"),
        Some(Number::Float(i)) => println!("Result is {i}"),
        _ => println!("Something went wrong"),
    };
}