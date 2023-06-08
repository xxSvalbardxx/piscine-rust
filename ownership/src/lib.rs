pub fn first_subword(mut s: String) -> String {
    let mut index = 0;
    for c in s.chars() {
        if c.is_uppercase() && index > 0 {
            break;
        } else if c == '_' && index > 0 {
            break;
        }
        index += 1;
    }
    s.truncate(index);
    s
}

