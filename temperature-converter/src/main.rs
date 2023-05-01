use std::io;

fn main() {
    loop {
        let mut temp_str = String::new();

        println!("Please enter the temperature");

        io::stdin()
            .read_line(&mut temp_str)
            .expect("Please enter valid temperature");

        let temp: f32 = match temp_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut convert_option_str = String::new();

        println!("Enter the option");
        println!("   1. Fahrenheit to Celsius");
        println!("   2. Celsius to Fahrenheit");
        println!("   3. Exit the program");

        io::stdin()
            .read_line(&mut convert_option_str)
            .expect("Please enter valid option");

        let option: i32 = match convert_option_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => println!("{}°F = {}°C", temp, convert_fahrenheit_to_celsius(temp)),
            2 => println!("{}°C = {}°F", temp, convert_celsius_to_fahrenheit(temp)),
            3 => break,
            _ => {
                println!("Invalid conversion");
                continue;
            }
        }

        println!("\n")
    }
}

fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}
