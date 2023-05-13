use pig_latin;

fn main() {
    let word_with_consonant = "first".to_string();

    let pig_latin_word = pig_latin::convert_word_to_pig_latin(word_with_consonant);
    println!("{pig_latin_word}");
    assert_eq!(pig_latin_word, "irst-fay");

    let word_with_vowel_as_first_character = String::from("apple");

    let pig_latin_word = pig_latin::convert_word_to_pig_latin(word_with_vowel_as_first_character);
    println!("{pig_latin_word}");
    assert_eq!(pig_latin_word, "apple-hay");
}
