pub fn talking(text: &str) -> &str {
    match text {
        _ if text.iter().all(|c| c.is_whitespace())|| text.len() == 0 || text == " " => "Just say something!",
        
        _ if text.to_uppercase() == text && text.ends_with("?") && text.len() > 0 => "Quiet, I am thinking!",
        _ if text.to_uppercase() == text && text.ends_with("!") && text.len() > 0 => "There is no need to yell, calm down!",
        _ if text.ends_with("?") && text.len() > 0 => "Sure.",
        _ => "Interesting",
    }
    
    
    
    
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
    } */
}


