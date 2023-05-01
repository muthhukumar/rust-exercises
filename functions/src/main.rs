fn main() {
    println!("Hello, world!");

    greet();
    display_number(10);
    print_addition(20, 30);

    println!("addition of {} and {} is {}", 1, 2, add(1, 2))
}

fn add(number_1: i32, number_2: i32) -> i32 {
    number_1 + number_2 // this evaluate to a value which is expression and the evaluated value will returned to the called code
}

fn greet() {
    println!("Hello!!");
}

fn display_number(number: i32) {
    println!("Number is {}", number)
}

fn print_addition(number_1: i32, number_2: i32) {
    println!("{} + {} = {}", number_1, number_2, number_1 + number_2)
}
