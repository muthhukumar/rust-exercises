fn main() {
    let mut i = 0;
    // loop {
    //     if i > 100 {
    //         break;
    //     }

    //     println!("{}", i);
    //     i = i + 1;
    // }

    while i <= 100 {
        println!("{}", i);
        i = i + 1;
    }

    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 5];

    loop_through_collection(numbers);
    loop_through_collection_using_for_loop(numbers);
    print_lift_off();
}

fn loop_through_collection(numbers: [i32; 6]) {
    let length: usize = numbers.len();

    let mut index: usize = 0;

    while index < length {
        println!("value is {} at index {}", numbers[index], index);
        index = index + 1;
    }
}

fn loop_through_collection_using_for_loop(numbers: [i32; 6]) {
    for number in numbers.iter() {
        println!("Number is {}", number)
    }
}

fn print_lift_off() {
    for number in (1..5).rev() {
        println!("{}", number);
    }

    println!("Lift off!!");
}
