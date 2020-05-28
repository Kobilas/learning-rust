use std::io;

const VOWELS: &str = "aeiou";

fn main() {
    println!("String to convert to pig latin:");
    let mut to_convert = String::new();
    loop {
        // check for numeral or invalid string?
        io::stdin().read_line(&mut to_convert)
            .expect("Failed to read line.");
        to_convert = to_convert.trim().to_string();
        if let false = to_convert.is_empty() {
            break;
        }
        /*match to_convert.is_empty() {
         *    false => break,
         *    true => to_convert = String::new(),
         *}
         */
        to_convert = String::new();
    }
    let converted = translate_to_pig_latin(&to_convert);
    println!("{} translated to pig latin: {}", to_convert, converted);
}

fn translate_to_pig_latin(s: &String) -> String {
    let mut to_append = String::from("-");
    let mut first_vowel = 0;
    for c in s.chars() {
        if VOWELS.find(c.to_ascii_lowercase()) != None {
            if first_vowel == 0 {
                to_append.push_str("hay");
            } else {
                to_append.push_str("ay");
            }
            break;
        } else {
            first_vowel += 1;
            to_append.push(c);
        }
    }
    let mut ret = String::new();
    ret.push_str(&s[first_vowel..]);
    ret.push_str(&to_append);
    ret
}
