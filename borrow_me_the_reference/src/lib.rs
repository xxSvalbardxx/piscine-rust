pub fn delete_and_backspace(s: &mut String) {
    let mut i = 0;
    while i < s.len() {
        if s.chars().nth(i).unwrap() == '-' { // nth() returns an Option<char>
            s.remove(i);
            if i > 0 {
                s.remove(i - 1);
                i -= 2;
            }
        } else {
            i += 1;
        }
    }
    let mut j = s.len()-1; 
    while j > 0 {
        if s.chars().nth(j).unwrap() == '+' {
            s.remove(j);
            if j < s.len() {
                s.remove(j);
            }
        } else {
            j -= 1;
        }
    }
}


pub fn do_operations(v: &mut Vec<String>) {
    for s in v {
        let mut i = 0;
        while i < s.len() {
            if s.chars().nth(i).unwrap() == '+' {
                let a = s[..i].parse::<i32>().unwrap();
                let b = s[i+1..].parse::<i32>().unwrap();
                let c = a + b;
                s.clear();
                s.push_str(&c.to_string());
                break;
            } else if s.chars().nth(i).unwrap() == '-' {
                let a = s[..i].parse::<i32>().unwrap();
                let b = s[i+1..].parse::<i32>().unwrap();
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
	let mut b: Vec<String> = vec!["2+2", "3+2", "10-3", "5+5"];

	delete_and_backspace(&mut a);
	do_operations(&mut b);

	println!("{:?}", (a, b));
}

}
