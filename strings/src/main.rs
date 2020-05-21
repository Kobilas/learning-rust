fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // actually string is empty, but word still contains in
    s.clear();
    let mut s = String::from("hello world word");
    let second_word = second_word_as_slice(&s);
    println!("{}", second_word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // return i early if we encounter a space
        }
    }
    s.len() // if no space encountered, we return the last index of the string
}

fn first_word_as_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word_as_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    let start_idx = 0;
    let end_idx: i32 = -1;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && start_idx == 0 {
            let start_idx = i;
        } else if item == b' ' && end_idx == -1 {
            let end_idx = i;
        }
    }
    if end_idx == -1 {
        return &s[..];
    }
    &s[start_idx+1..end_idx as u32]
}