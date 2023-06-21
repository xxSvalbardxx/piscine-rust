pub fn num_to_ordinal(x: u32) -> String {
    let mut s = String::new();
    match x {
        1 => s.push_str("st"),
        2 => s.push_str("nd"),
        3 => s.push_str("rd"),
        _ => s.push_str(&format!("{}th", x)),
    }
    s
}