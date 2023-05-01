use std::io;

fn main() {
    let mut n_str = String::new();

    loop {
        println!("Please enter the nth of the Fibonacci number you would like to find:");

        io::stdin()
            .read_line(&mut n_str)
            .expect("Enter valid nth number");

        let n: i32 = match n_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid number");
                continue;
            }
        };
        println!("nth finbonacci number is {}", find_nth_fibonacci_number(n));
    }
}

fn find_nth_fibonacci_number(n: i32) -> i32 {
    let mut a: i32 = -1;
    let mut b: i32 = 1;
    let mut c: i32 = 0;

    for _ in 0..n + 1 {
        c = a + b;
        a = b;
        b = c;
    }

    c
}
