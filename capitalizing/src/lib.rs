pub fn capitalize_first(input: &str) -> String {
    for i in input.chars() {
        if i.is_alphabetic() {
            let mut new_word = String::new();
            new_word.push(i.to_ascii_uppercase());
            new_word.push_str(&input[1..]);
            return new_word;
        }
    }
    return "".to_string();
}

pub fn title_case(input: &str) -> String {
    let mut res = String::new();
    for word in input.split_whitespace() {
        res.push_str(&capitalize_first(word));
        res.push(' ');
    }
    res.pop();
    res
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for i in input.chars() {
        if i.is_alphabetic() {
            if i.is_uppercase() {
                res.push(i.to_ascii_lowercase());
            } else {
                res.push(i.to_ascii_uppercase());
            }
        } else {
            res.push(i);
        }
    }
    res
}