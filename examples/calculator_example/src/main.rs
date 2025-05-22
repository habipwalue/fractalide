// Simple main.rs to run the calculator example
use std::io;

fn main() {
    println!("Fractalide Calculator Example");
    println!("============================");
    println!("This is a simplified example without the full Fractalide runtime");
    println!("In a real implementation, this would use the Fractalide dataflow architecture");
    
    // Simulate the calculator
    simulate_calculator();
}

fn simulate_calculator() {
    loop {
        println!("\nEnter first number (or 'q' to quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let input = input.trim();
        if input == "q" {
            break;
        }
        
        let num1: f64 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue;
            }
        };
        
        println!("Enter second number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let num2: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue;
            }
        };
        
        println!("Enter operation (+, -, *, /):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let operation = input.trim();
        
        // Simulate node processing
        println!("\n[Node: Input] Received numbers: {} and {}", num1, num2);
        println!("[Node: Input] Received operation: {}", operation);
        
        let result = match operation {
            "+" => {
                println!("[Node: Add] Adding {} and {}", num1, num2);
                Ok(num1 + num2)
            },
            "-" => {
                println!("[Node: Subtract] Subtracting {} from {}", num2, num1);
                Ok(num1 - num2)
            },
            "*" => {
                println!("[Node: Multiply] Multiplying {} and {}", num1, num2);
                Ok(num1 * num2)
            },
            "/" => {
                println!("[Node: Divide] Dividing {} by {}", num1, num2);
                if num2 == 0.0 {
                    Err("Division by zero error".to_string())
                } else {
                    Ok(num1 / num2)
                }
            },
            _ => Err("Invalid operation".to_string())
        };
        
        match result {
            Ok(value) => {
                println!("[Node: Display] Result: {}", value);
            },
            Err(error) => {
                println!("[Node: Error] Error: {}", error);
            }
        }
    }
    
    println!("Calculator closed.");
}