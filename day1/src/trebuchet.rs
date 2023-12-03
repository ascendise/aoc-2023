use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn recover(s: &str) -> u32 {
    let converted = replace_word_with_digit(s);
    let first_char = converted.chars().find(|c| c.is_numeric()).unwrap();
    let last_char = converted.chars().rfind(|c| c.is_numeric()).unwrap();
    let number = format!("{first_char}{last_char}");
    number.trim().parse().unwrap()
}

fn replace_word_with_digit(s: &str) -> String {
    let digits: HashMap<&str, &str> = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e")
    ]);
    let mut new_string = String::from(s);
    for digit in digits {
        new_string = new_string.replace(digit.0, digit.1);
    }
    new_string
}