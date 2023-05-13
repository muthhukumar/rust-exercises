// Problem statement
// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {
    let mut integers = vec![2, 4, 6, 3, 100, 9, 8, 10, 40, 20, 2, 4, 8];

    integers.sort();

    let middle_element_index = integers.len() / 2;

    let middle_element = integers[middle_element_index];

    println!("Vector elements...");

    for i in &integers {
        println!("{i}");
    }

    println!();

    let mut map = HashMap::new();

    let mut max_count = 0;
    let mut max_element = 0;

    for i in &integers {
        let count = map.entry(i).or_insert(0);
        *count = *count + 1;

        if *count > max_count {
            max_count = *count;
            max_element = *i;
        }
    }

    println!("Result:");
    println!("Middle element index: {middle_element_index}");
    println!("Middle element(median): {middle_element}");
    println!("Maximum repeated element(mode): {max_element}");
    println!("Max element count: {max_count}");

    for (key, value) in &map {
        println!("{key}: {value}");
    }

    println!();
}
