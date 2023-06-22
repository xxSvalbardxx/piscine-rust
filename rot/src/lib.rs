pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    for c in input.chars() {
        let rotated_char = match c {
            'a'..='z' => {
                let base = if c.is_lowercase() { b'a' } else { b'A' };
                let offset = ((c as u8 - base) as i8 + key).rem_euclid(26) as u8;
                (base + offset) as char
            }
            'A'..='Z' => {
                let base = if c.is_lowercase() { b'a' } else { b'A' };
                let offset = ((c as u8 - base) as i8 + key).rem_euclid(26) as u8;
                (base + offset) as char
            }
            _ => c,
        };

        result.push(rotated_char);
    }

    result
}