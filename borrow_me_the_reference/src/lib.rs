pub fn delete_and_backspace(s: &mut String) {
    let mut i = 0;
    while i < s.len() {
        if s.chars().nth(i).unwrap() == '-' {
            // nth() returns an Option<char>
            s.remove(i);
            if i > 0 {
                s.remove(i - 1);
                i -= 2;
            }
        } else {
            i += 1;
        }
    }
    let mut j = s.len() - 1;
    while j > 0 {
        if s.chars().nth(j) == Some('+') {
            // Some is an Option variant wich contains a value
            s.remove(j);
            if j < s.len() {
                s.remove(j);
            }
        } else {
            j -= 1;
        }
    }
    while s.chars().nth(0) == Some('+') {
        if s.len() == 1 {
            s.remove(0);
            break;
        } else {
            s.remove(0);
            s.remove(0);
        }
    }
}

pub fn do_operations(v: &mut Vec<String>) {
    for s in v {
        let mut i = 0;
        while i < s.len() {
            if s.chars().nth(i).unwrap() == '+' {
                let a = s[..i].parse::<i32>().unwrap();
                let b = s[i + 1..].parse::<i32>().unwrap();
                let c = a + b;
                s.clear();
                s.push_str(&c.to_string());
                break;
            } else if s.chars().nth(i).unwrap() == '-' {
                let a = s[..i].parse::<i32>().unwrap();
                let b = s[i + 1..].parse::<i32>().unwrap();
                let c = a - b;
                s.clear();
                s.push_str(&c.to_string());
                break;
            } else {
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use borrow_me_the_reference::{delete_and_backspace, do_operations};

    fn main() {
        let mut a = String::from("bpp--o+er+++sskroi-++lcw");
        let mut b: Vec<String> = vec!["2+2".to_string(), "3+2".to_string(), "10-3".to_string(), "5+5".to_string()];

        delete_and_backspace(&mut a);
        do_operations(&mut b);

        println!("{:?}", (a, b));
    }
}
