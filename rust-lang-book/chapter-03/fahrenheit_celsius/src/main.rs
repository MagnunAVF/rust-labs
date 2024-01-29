use std::io;

fn fahrenheit_to_celsius(degree: f64) -> f64 {
    (degree - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(degree: f64) -> f64 {
    degree * 9.0 / 5.0 + 32.0
}

fn print_menu() {
    println!("\nChoose a conversion:");
    println!("   * (1) °F to °C");
    println!("   * (2) °C to °F");
    println!("   * (q) quit");
}

fn get_temperature(d_type: &str) -> Result<f64, String> {
    println!("Please input {} degrees:", d_type);

    let mut degree = String::new();

    io::stdin()
        .read_line(&mut degree)
        .map_err(|err| err.to_string())?;

    let degree = degree
        .trim()
        .parse::<f64>()
        .map_err(|err| err.to_string())?;

    Ok(degree)
}

fn main() {
    println!("\n*** Temperature degree conversion ***");

    loop {
        print_menu();

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read the selected option");

        let option: char = match option.trim().parse() {
            Ok(char) => char,
            Err(_) => {
                println!("Invalid option!");
                continue;
            }
        };

        match option {
            '1' => {
                println!("Converting fahrenheit to celsius.");

                let input = get_temperature("°F");
                match input {
                    Ok(degree) => {
                        let result = fahrenheit_to_celsius(degree);
                        println!("Result: {}", result);
                    }
                    Err(_error) => {
                        println!("Invalid degree.");
                        continue;
                    }
                }
            }
            '2' => {
                println!("Converting celsius to fahrenheit.");

                let input = get_temperature("°C");
                match input {
                    Ok(degree) => {
                        let result = celsius_to_fahrenheit(degree);
                        println!("Result: {}", result);
                    }
                    Err(_error) => {
                        println!("Invalid degree.");
                        continue;
                    }
                }
            }
            'q' => {
                println!("Bye!");
                break;
            }
            _ => {
                println!("Invalid option!");
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(50.43), 10.238888888888889);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(32.67), 90.80600000000001);
    }
}
