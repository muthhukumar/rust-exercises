use std::collections::HashMap;

pub type Department = HashMap<String, Vec<String>>;

pub fn push_employee_detail(text: String, map: &mut Department) {
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

pub fn print_department_staffs(map: &Department) {
    for (key, value) in map {
        println!("{key} department staffs");

        for name in value {
            println!("{name}")
        }
    }
}
