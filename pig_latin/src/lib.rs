mod letters;

fn remove_and_return_first_consonant(word: String) -> (String, Option<char>) {
    let mut final_str = "".to_string();

    let mut found_first_consonant = false;

    let mut first_consonant: Option<char> = None;

    for letter in word.chars() {
        if !found_first_consonant && letters::CONSONANT_LETTERS.contains(&letter) {
            found_first_consonant = true;
            first_consonant = Some(letter);
        } else {
            final_str = format!("{final_str}{letter}");
        }
    }

    (final_str, first_consonant)
}

fn attach_consonant_and_ay(word: String, consonant: Option<char>) -> String {
    match consonant {
        Some(value) => format!("{word}-{value}ay"),
        None => format!("{word}-ay"),
    }
}

fn attach_hay_to_vowel_word(word: String) -> String {
    // format!("{word}-hay");
    word + "-" + "hay"
}

fn is_first_letter_vowel(word: &String) -> bool {
    match word.chars().next() {
        Some(value) => letters::VOWEL_LETTERS.contains(&value),
        None => false,
    }
}

pub fn convert_word_to_pig_latin(word: String) -> String {
    match is_first_letter_vowel(&word) {
        true => attach_hay_to_vowel_word(word),
        false => {
            let (consonant_remove_word, first_consonant) = remove_and_return_first_consonant(word);

            attach_consonant_and_ay(consonant_remove_word, first_consonant)
        }
    }
}
