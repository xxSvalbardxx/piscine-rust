pub fn talking(text: &str) -> &str {
    /* if text.len() == 0 || text == " " {
        return "Just say something!";
    } else if text.to_uppercase() == text && text.ends_with("?") && text.len() > 0 {
        return "Quiet, I am thinking!";
    } else if text.to_uppercase() == text && text.ends_with("!") && text.len() > 0 {
        return "There is no need to yell, calm down!";
    } else if text.ends_with("?") && text.len() > 0 {
        return "Sure.";
    } else {
        return "Interesting";
    }
    // remove the last character if it is'nt a letter
    let text_1 = match text.chars().last() {
        Some(c) if !c.is_alphabetic() => &text[..text.len()-1],
        _ => text,
    };
    match text {
        _ if text_1.chars().all(|c| c.is_whitespace())|| text.len() == 0 || text == " " => "Just say something!",
        _ if text_1.chars().all(|c| c.is_uppercase()) && text.ends_with("?") && text.len() > 0 => "Quiet, I am thinking!",
        _ if text_1.chars().all(|c| c.is_uppercase()) && text.ends_with("!") && text.len() > 0 => "There is no need to yell, calm down!",
        _ if text_1.ends_with("?") && text.len() > 0 => "Sure.",
        _ => "Interesting",
    } */
    // it does'nt work with this code so i will do it like this :

    if text.len() == 0 || text == " " {
        return "Just say something!";
    } else if text.to_uppercase() == text && text.ends_with("?") && text.len() > 0 {
        return "Quiet, I am thinking!";
    } else if text.to_uppercase() == text && text.ends_with("!") && text.len() > 0 {
        return "There is no need to yell, calm down!";
    } else if text.ends_with("?") && text.len() > 0 {
        return "Sure.";
    } else {
        return "Interesting";
    }

}


