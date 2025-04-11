use std::io;

fn main() {
    println!("Enter the temperature to convert (e.g., 30C or 86F):");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    let len = input.len();
    
    if len < 2 {
        println!("Invalid input format. Please use something like 30C or 86F.");
        return;
    }

    let (value_part, unit_part) = input.split_at(len - 1);

    let value: f64 = match value_part.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number format.");
            return;
        }
        
    };

    let unit = unit_part.to_uppercase();

    match unit.as_str() {
        "C" => {
            let fahrenheit = (value * 9.0 / 5.0) + 32.0;
            println!("{:.2}C is {:.2}F", value, fahrenheit);
        }
        "F" => {
            let celsius = (value - 32.0) * 5.0 / 9.0;
            println!("{:.2}F is {:.2}C", value, celsius);
        }
        _ => {
            println!("Unknown unit. Please use 'C' for Celsius or 'F' for Fahrenheit.");
        }
    }
    

    
}

