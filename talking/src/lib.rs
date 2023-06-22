
pub fn talking(text: &str) -> String {
    
    let mut lowercase_count = 0;
    let mut uppercase_count = 0;
    let mut res = String::new();
    for i in text.chars() {
        lowercase_count += match i {
            'a'..='z' => 1,
            _ => 0,
        };
        uppercase_count += match i {
            'A'..='Z' => 1,
            _ => 0,
        };
    }
    if lowercase_count == 0 && uppercase_count != 0 && text.chars().nth(text.len()-1).unwrap() == '?' {
        res = "Quiet, I am thinking!".to_string();
    } else if lowercase_count == 0 && uppercase_count != 0 {
        res = "There is no need to yell, calm down!".to_string();
    } else if text.len() > 0 && s.chars().nth(text.len()-1).unwrap() == '?' {
        res = "Sure.".to_string();
    } else if lowercase_count == 0 && uppercase_count == 0 || text.len() == 0 {
        res = "Just say something!".to_string();
    } else {
        res = "Interesting".to_string();
    }
    res
}



