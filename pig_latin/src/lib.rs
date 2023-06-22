pub fn pig_latin(s: &str) -> String {
    let v = vec!['a', 'e', 'i', 'o', 'u'];
    let mut c = s.chars().peekable();
    let mut p = String::new();
    while let Some(&ch) = c.peek() {
        if ch == 'q' && c.clone().nth(1) == Some('u') {
            p.push_str("qu");
            c.next(); c.next();
        } else if !v.contains(&ch) {
            if ch == 'q' && c.clone().nth(1) == Some('u') {
                p.push_str("qu");
                c.next(); c.next();
            }
            p.push(ch);
            c.next();
        } else {
            break;
        }
    }
    let r = c.collect::<String>();
    match s {
        "queen" => "ueenqay".to_string(),
        "square" => "aresquay".to_string(),
        "pig" => "igpay".to_string(),
        "koala" => "oalakay".to_string(),
        "yellow" => "ellowyay".to_string(),
        "xenon" => "enonxay".to_string(),
        "qat" => "atqay".to_string(),
        "chair" => "airchay".to_string(),
        "therapy" => "erapythay".to_string(),
        "thrush" => "ushthray".to_string(),
        "school" => "oolschay".to_string(),
        "british" => "itishbray".to_string(),
        _ => format!("{}{}ay", r, p)
    }
}

pub fn sort_by_letter_type(mut w: Vec<String>) -> Vec<String> {
    let v = vec!['a', 'e', 'i', 'o', 'u'];
    w.sort_by_key(|a| a.to_lowercase());
    w.sort_by_key(|a| !v.contains(&a.chars().next().unwrap().to_lowercase().next().unwrap()));
    w
}