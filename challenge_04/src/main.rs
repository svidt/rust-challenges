fn main() {
    let input: String = String::from(" Hello, world! ");
    println!("{}", trim_spaces(&input));
}

fn trim_spaces(s: &str) -> &str {
    // Locates the first num-space character
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    // Search in reverse to locate the last non-space character
    let mut end = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]
}
