use std::collections::HashMap;
use std::io;

type Department = HashMap<String, Vec<String>>;

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

                for (key, value) in &map {
                    println!("{key} department staffs");

                    for name in value {
                        println!("{name}")
                    }
                }
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

fn push_employee_detail(text: String, map: &mut Department) {
    match get_employee_name_and_department(text) {
        Ok((name, department)) => {
            let department_list = map.entry(department).or_insert(Vec::new());

            department_list.push(name);

            println!("Details added successfully");
        }
        Err(message) => {
            println!("{message}");
        }
    }
}

fn get_employee_name_and_department(text: String) -> Result<(String, String), &'static str> {
    let parts: Vec<&str> = text.split_whitespace().collect();

    let len = parts.len();

    if len != 4 {
        return Err("Invalid prompt");
    }

    let add = parts[0];
    let name = parts[1];
    let to = parts[2];
    let department = parts[3];

    if add.to_lowercase() != "add"
        || to.to_lowercase() != "to"
        || name.is_empty()
        || department.is_empty()
    {
        return Err("Invalid prompt");
    }

    Ok((name.to_string(), department.to_string()))
}
