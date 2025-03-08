use std::str::Chars;

/// Translate a word in pig-latin
/// Rueles:
///  1. if the word start with a consonat, it is moved to the end of the word
///   and 'ay' is added
///  2. if the word start with a vowel 'hay' is added to the end
///
pub fn translate(word: &str) -> String {
    let mut chars = word.chars();
    match chars.nth(0) {
        Some(c) =>  apply_translation(c, chars),
        None => String::new()
    }
}

fn apply_translation(c: char, chars: Chars) -> String {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' =>{
        let mut traslated = String::from(c);
            traslated.extend(chars);
            traslated.push_str("hay");
            traslated
        }
        _ => {
            let mut translated = String::from_iter(chars);
            translated.push(c);
            translated.push_str("ay");
            translated
        }
    }
}

