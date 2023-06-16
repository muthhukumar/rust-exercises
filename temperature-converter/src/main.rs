use std::io;
use temperature_converter::*;

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
            1 => {
                println!("{}°F = {}", temp, Celsius::from(Fahrenheit(temp)))
            }
            2 => println!("{}°C = {}", temp, Fahrenheit::from(Celsius(temp))),
            3 => break,
            _ => {
                println!("Invalid conversion");
                continue;
            }
        }

        println!("\n")
    }
}
