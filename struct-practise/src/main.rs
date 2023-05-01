struct User {
    email: String,
    username: String,
    active: bool,
}

struct RGB(i32, i32, i32);

struct Color {
    rgb: RGB,
    name: String,
}

// Unit like struct
struct EmptyStruct {}

fn main() {
    let user = User {
        email: String::from("test@test.com"),
        username: String::from("username"),
        active: false,
    };

    let mut current_user = User {
        email: String::from("currentuser@gmail.com"),
        username: String::from("username2"),
        active: false,
    };

    let new_user = build_user(String::from("final@test.com"), String::from("finaltest"));

    update_username(&mut current_user);

    let final_user = User {
        email: String::from("finaluser@gmail.com"),
        username: String::from("finaluser"),
        ..new_user // Struct update syntax
    };

    if final_user.active == new_user.active {
        println!("Both new user and final user active is same");
    }

    let red_color = Color {
        name: String::from("red"),
        rgb: RGB(255, 0, 0),
    };

    print_user(&user);
    print_user(&current_user);
    print_user(&new_user);
    print_color(&red_color);
}

fn print_color(color: &Color) {
    println!(
        "color: {}, rgb: ({}, {}, {})",
        color.name, color.rgb.0, color.rgb.1, color.rgb.2
    )
}

fn build_user(email: String, username: String) -> User {
    User {
        // Here we are using field init shorthand syntax to avoid rewriting email: email. It
        // is possible because the parameter name and the field name is same. So it is good to name
        // parameter same as field name
        email,
        username,
        active: true,
    }
}

// here we are getting the user value as reference. Because we don't want to trasfer the ownership to the function
fn print_user(user: &User) {
    println!(
        "email: {}, username: {}, active: {}",
        user.email, user.username, user.active
    );
}

fn update_username(user: &mut User) -> &User {
    user.username = String::from("updated_username");

    user
}
