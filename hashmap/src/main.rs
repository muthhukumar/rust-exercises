use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    scores.insert(23.to_string(), 40);
    scores.insert("Green".to_string(), 40);

    let blue_team_key = String::from("Blue");
    let blue_team_score = scores.get(&blue_team_key).copied().unwrap_or(0);

    println!("{:?}", scores.get(&String::from("Blue")));
    println!("{:?}", scores.get(&String::from("Red")));
    println!("{:?}", scores.get(&23.to_string()));
    println!("{:?}", blue_team_score);

    let reference = scores.entry(String::from("Green")).or_insert(80);

    println!("reference: {reference}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut map = HashMap::new();

    let field_key = String::from("Key");
    let field_value = String::from("Value");

    map.insert(field_key, field_value);

    update_value_based_on_old_value();
}

fn update_value_based_on_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    let mut i = 0;

    for word in text.split_whitespace() {
        println!("word = {word}");
        i += 1;
        let count = map.entry(word).or_insert(0);
        println!("count = {count}");
        *count = i;
    }

    for (key, value) in &map {
        println!("{key}: {value}");
    }

    println!("{:?}", map);
}
