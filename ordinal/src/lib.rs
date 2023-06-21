pub fn num_to_ordinal(x: u32) -> String {
    let mut s = String::new();
    match x {
        1 => s.push_str(&format!("{}st", x)),
        2 => s.push_str(&format!("{}nd", x)),
        3 => s.push_str(&format!("{}rd", x)),
        _ => s.push_str(&format!("{}th", x)),
    }
    s
}