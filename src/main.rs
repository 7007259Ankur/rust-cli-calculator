use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: cargo run <operation> <num1> <num2>");
        return;
    }

    let operation = &args[1];

    let a: i32 = args[2].parse().unwrap();
    let b: i32 = args[3].parse().unwrap();

    let result = match operation.as_str() {
        "add" => a + b,
        "sub" => a - b,
        "mul" => a * b,
        "div" => a / b,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    println!("Result: {}", result);
}