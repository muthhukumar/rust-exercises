use std::collections::HashMap;
use std::io;

use store_employee_details::{print_department_staffs, push_employee_detail, Department};

fn main() {
    let mut map: Department = HashMap::new();

    loop {
        let mut prompt = String::new();
        let mut option = String::new();

        println!();

        println!("Enter the option");

        println!("  1. Make a new entry using the following prompy(Add <name> to <department>)");
        println!("  2. List out all the people from a department");
        println!("  3. List out all the department and department people");
        println!("  4. Exit");

        println!();

        io::stdin()
            .read_line(&mut option)
            .expect("Enter valid option");

        let option: i32 = match option.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        match option {
            1 => {
                println!("Enter the prompt in (Add <name> to <department>) format");

                io::stdin()
                    .read_line(&mut prompt)
                    .expect("Enter valid prompt");

                push_employee_detail(prompt, &mut map);
            }
            2 => {
                println!("Enter the department");

                let mut department = String::new();

                io::stdin()
                    .read_line(&mut department)
                    .expect("Enter valid department");

                println!();

                let department_people = map.get(department.trim());

                println!("Staff from {department} are...");

                println!();

                match department_people {
                    Some(value) => {
                        for people in value {
                            println!("{people}");
                        }
                    }
                    None => println!("No staff found for the {department} department."),
                }
            }
            3 => {
                if map.len() <= 0 {
                    println!("No departments found. Please add some.");

                    continue;
                }

                println!("Listing all the departments and their staffs...");

                print_department_staffs(&map);
            }
            4 => {
                println!("Excited.");
                break;
            }
            _ => {
                println!("Enter valid option")
            }
        }
    }
}
