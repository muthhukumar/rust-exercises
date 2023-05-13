#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    plus_one(Some(5));
    plus_one(None);

    dice_example(3);
    dice_example(7);
    dice_example(10);

    if_let_syntax();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // Match arms
        //syntax: Pattern => { some code to handle the pattern case }
        Coin::Penny => {
            println!("Feelin Lucky");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::Some(value) => Some(value + 1),
        None => None,
    }
}

fn dice_example(roll: u8) {
    match roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // Here we have used the variable instead of specifying the exact value we have to match.
        // Because of that it becomes match all pattern case. If we want to use the variable value
        // we can name the variable but if we don't want to use the value but still want to match
        // all case then we can use _. ->
        // _ => move_player(10) here _ does not bind the value of the match is called with
        // _ => (), - by adding this we are telling don't do anything
        something => move_player(something), // So the last one is match all pattern. Match all pattern should be the last one. Because each arms executes in order from top to bottom
    }

    fn add_fancy_hat() {
        println!("Add fancy hat");
    }
    fn remove_fancy_hat() {
        println!("Remove fancy hat");
    }
    fn move_player(num_spaces: u8) {
        println!("Move {} spaces", num_spaces);
    }
}

fn if_let_syntax() {
    let x = Some(5);

    let y: Option<i32> = None;

    if let Some(value) = x {
        println!("value is {}", value);
    }

    if let None = y {
        println!("value is none");
    }
}
