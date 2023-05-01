fn main() {
    let number = 4;

    if number < 5 {
        println!("{} is less than 5", number);
    } else {
        println!("{} is greater than 5", number)
    }

    println!("9 is divisible by {}", is_divisible_by(9));

    let y = if number % 3 == 0 { 3 } else { 2 };

    println!("value of y is {}", y)
}

fn is_divisible_by(number: i32) -> i32 {
    if number % 2 == 0 {
        2
    } else if number % 3 == 0 {
        3
    } else {
        0
    }
}
