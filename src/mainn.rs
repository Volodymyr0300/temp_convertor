use std::io;

fn main() {
    loop {
        println!("Enter the temperature to convert (e.g., 30C or 86F), or type 'q' to quit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        match convert_temperature(input) {
            Some(result) => println!("{}", result),
            None => println!("Invalid input. Try something like 35C or 86F."),
        }
    }
}

fn convert_temperature(input: &str) -> Option<String> {
    if input.len() < 2 {
        return None;
    }

    let (value_part, unit_part) = input.split_at(input.len() - 1);
    let value: f64 = value_part.trim().parse().ok()?;
    let unit = unit_part.trim().to_uppercase();

    match unit.as_str() {
        "C" => {
            let f = (value * 9.0 / 5.0) + 32.0;
            Some(format!("{:.2}C is {:.2}F", value, f))
        }
        "F" => {
            let c = (value - 32.0) * 5.0 / 9.0;
            Some(format!("{:.2}F is {:.2}C", value, c))
        }
        _ => None,
    }
}
