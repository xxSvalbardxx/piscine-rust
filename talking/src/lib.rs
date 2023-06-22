pub fn talking(text: &str) -> &str {
    if text.len() == 0 {
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


