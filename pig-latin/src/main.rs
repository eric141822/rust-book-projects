use std::io;
fn main() {
    // function that turns word into pig latin.
    let mut word = String::new();
    println!("Enter a word to turn into pig latin: ");
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let word = word.trim();
    println!("{} in pig latin is {}", word, convert(word));
}

fn starts_with_vowel(s: &str) -> bool {
    if let Some(first_char) = s.chars().next() {
        matches!(first_char, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    } else {
        false
    }
}
// convert into pig latin
fn convert(word: &str) -> String {
    let is_first_char_vowel = starts_with_vowel(word);
    if is_first_char_vowel {
        let word = format!("{}-hay", word);
        word
    } else {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        let rest = chars.as_str();
        let word = format!("{}-{}ay", rest, first_char);
        word
    }
}
