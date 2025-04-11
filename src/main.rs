use std::io;

fn main() {
    loop {
    println!("Enter the temperature to convert (e.g., 30C or 86F):");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
        println!("Goodbye!");
        break;
    }

    if input.is_empty() || input.len() < 2 {
        println!("Invalid input format. Please use something like 30C or 86F.");
        continue;
    }

    let len = input.len();

    let (value_part, unit_part) = input.split_at(len - 1);

    let value: f64 = match value_part.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number format. Please try again.");
            return;
        }
        
    };

    let unit = unit_part.trim().to_uppercase();

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
            println!("Unknown unit '{}'. Use 'C' or 'F'.", unit);
        }
    }
    }

}

